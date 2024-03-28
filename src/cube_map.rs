use crate::resources::ResourceLoader;

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
