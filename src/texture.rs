use crate::resources::ResourceLoader;

#[derive(Clone)]
pub struct Texture {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Texture {
    fn new(gl: &gl::Gl, img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Texture {
        let mut id = 0;

        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);

            let target = gl::TEXTURE_2D;
            gl.TexParameteri(target, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(target, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(
                target,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr().cast(),
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }

        Texture { gl: gl.clone(), id }
    }

    pub fn load(gl: &gl::Gl, res: &ResourceLoader, path: &str) -> Result<Texture, String> {
        let img = res
            .load_image(path)
            .map_err(|e| format!("Error loading image {}: {:?}", path, e))?;

        Ok(Texture::new(gl, img))
    }

    // pub fn from_binary_data(gl: &gl::Gl, data: &[u8]) -> Result<Texture, String> {
    //     let mut reader = image::io::Reader::new(std::io::Cursor::new(data));
    //     reader.set_format(image::ImageFormat::Png);
    //     // reader.no_limits();
    //     let img = reader.decode().map_err(|_| "Hey")?.flipv().to_rgba8();

    //     Ok(Texture::new(gl, img))
    // }

    pub fn bind(&self, slot: gl::types::GLenum) {
        unsafe {
            self.gl.ActiveTexture(slot);
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, [self.id].as_ptr()) };
    }
}

pub struct CubeMap {
    gl: gl::Gl,
    pub id: gl::types::GLuint,
}

impl CubeMap {
    pub fn new(gl: &gl::Gl, images: [image::ImageBuffer<image::Rgb<u8>, Vec<u8>>; 6]) -> CubeMap {
        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_CUBE_MAP, id);

            let target = gl::TEXTURE_CUBE_MAP;
            gl.TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(target, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
        };

        for (index, image) in images.iter().enumerate() {
            unsafe {
                gl.TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + index as u32,
                    0,
                    gl::RGB as i32,
                    image.width() as i32,
                    image.height() as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    image.as_ptr().cast(),
                );
            }
        }

        CubeMap { gl: gl.clone(), id }
    }

    pub fn load(
        gl: &gl::Gl,
        res: &ResourceLoader,
        images_paths: [&str; 6],
    ) -> Result<CubeMap, String> {
        let load_image = |path| {
            res.load_jpg(path)
                .map_err(|e| format!("Error loading image {}: {:?}", path, e))
                .unwrap()
        };

        let images = images_paths.map(load_image);
        Ok(CubeMap::new(gl, images))
    }
}
