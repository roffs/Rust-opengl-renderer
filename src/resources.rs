use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use image::ImageBuffer;
use image::{io::Reader, ImageError};

use crate::mesh::{Mesh, MeshVertex};
use crate::model::Model;
use crate::texture::Texture;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FileContainsNil(std::ffi::NulError),
    FailedToGetExePath,
    FailedToDecodeImage(ImageError),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}
impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Self {
        Error::FileContainsNil(other)
    }
}

impl From<ImageError> for Error {
    fn from(other: ImageError) -> Self {
        Error::FailedToDecodeImage(other)
    }
}

pub struct ResourceLoader {
    root_path: PathBuf,
}

impl ResourceLoader {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<ResourceLoader, Error> {
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;

        Ok(ResourceLoader {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn load_binary(&self, resource_path: &Path) -> Result<Vec<u8>, std::io::Error> {
        let path = relative_to_absolute_resource_path(&self.root_path, resource_path);
        std::fs::read(path)
    }

    pub fn load_cstring(&self, resource_path: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(relative_to_absolute_resource_path(
            &self.root_path,
            Path::new(resource_path),
        ))?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        let result = ffi::CString::new(buffer)?;

        Ok(result)
    }

    pub fn load_image(
        &self,
        resource_path: &str,
    ) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, Error> {
        let absolute_path =
            relative_to_absolute_resource_path(&self.root_path, Path::new(resource_path));
        let img = Reader::open(absolute_path)?.decode()?.to_rgba8();

        Ok(img)
    }

    pub fn load_model<'a>(&'a self, gl: &gl::Gl, resource_path: &str) -> Model {
        let relative_path = std::path::Path::new(resource_path);
        let current_directory = relative_path.parent().unwrap();

        let file = fs::File::open(resource_path).unwrap();
        let reader = io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();

        // Load buffers
        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    let binary_data = self
                        .load_binary(&relative_to_absolute_resource_path(
                            &self.root_path,
                            &current_directory.join(uri),
                        ))
                        .expect("Failed to load binary");
                    buffer_data.push(binary_data);
                }
                gltf::buffer::Source::Bin => {
                    if let Some(blob) = gltf.blob.as_deref() {
                        buffer_data.push(blob.into())
                    };
                }
            }
        }

        let mut textures = Vec::new();
        for texture in gltf.textures() {
            match texture.source().source() {
                gltf::image::Source::View { view, mime_type } => {
                    println!("Texture view: {:?}", view)
                }
                gltf::image::Source::Uri { uri, mime_type: _ } => textures.push(
                    Texture::load(
                        gl,
                        self,
                        relative_to_absolute_resource_path(
                            &self.root_path,
                            &current_directory.join(uri),
                        )
                        .to_str()
                        .unwrap(),
                    )
                    .unwrap(),
                ),
            };
        }

        let mut meshes = Vec::new();

        for mesh in gltf.meshes() {
            let mut mesh_vertices = Vec::new();
            let mut mesh_indices = Vec::new();

            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                // Read vertices and their attributes
                if let (Some(positions), Some(uvs)) = (
                    reader.read_positions(),
                    reader.read_tex_coords(0).map(|v| v.into_f32()),
                ) {
                    positions.zip(uvs).for_each(|(pos, uv)| {
                        mesh_vertices
                            .push(MeshVertex::new((pos[0], pos[1], pos[2]), (uv[0], uv[1])))
                    });
                }

                // Read vertex indices
                if let Some(indices) = reader.read_indices() {
                    indices
                        .into_u32()
                        .for_each(|index| mesh_indices.push(index as i32));
                }
            }

            meshes.push((Mesh::create(gl, mesh_vertices, mesh_indices), 0));
        }

        Model::new(meshes, textures)
    }
}

fn relative_to_absolute_resource_path(root_dir: &Path, location: &Path) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for dir in location.iter() {
        path = path.join(dir)
    }

    path
}
