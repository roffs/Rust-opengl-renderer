use anyhow::Result;
use image::io::Reader;

pub struct Texture {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn load(gl: &gl::Gl, path: &str) -> Result<Texture> {
        let img = Reader::open(path)?.decode()?.flipv().to_rgba8();

        let mut id: gl::types::GLuint = 0;
        unsafe { gl.GenTextures(1, &mut id) };

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, id);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
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

        Ok(Texture { gl: gl.clone(), id })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
