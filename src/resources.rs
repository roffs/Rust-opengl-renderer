use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use image::ImageBuffer;
use image::{io::Reader, ImageError};

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

    pub fn load_cstring(&self, resource_path: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(relative_to_absolute_resource_path(
            &self.root_path,
            resource_path,
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
        let absolute_path = relative_to_absolute_resource_path(&self.root_path, resource_path);
        let img = Reader::open(absolute_path)?.decode()?.flipv().to_rgba8();

        Ok(img)
    }
}

fn relative_to_absolute_resource_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split('/') {
        path = path.join(part);
    }

    path
}
