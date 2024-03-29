mod program;

use crate::resources::ResourceLoader;

pub use self::program::Program;

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_source(
        gl: &gl::Gl,
        res: &ResourceLoader,
        file_path: &str,
        kind: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let source = res
            .load_cstring(file_path)
            .map_err(|e| format!("Error loading shader {}: {:?}", file_path, e))?;

        let shader_id = unsafe { gl.CreateShader(kind) };

        let shader_type = match kind {
            gl::VERTEX_SHADER => "Vertex",
            gl::FRAGMENT_SHADER => "Fragment",
            _ => "Unspecified type",
        };

        unsafe {
            gl.ShaderSource(shader_id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(shader_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut log_len = 0_i32;
            let mut info_log: Vec<u8> = Vec::with_capacity(1024);

            unsafe {
                gl.GetShaderInfoLog(shader_id, 512, &mut log_len, info_log.as_mut_ptr().cast());
                info_log.set_len(log_len.try_into().unwrap());
            }

            return Err(format!(
                "Error: {} shader from {} compilation failed: {}",
                shader_type,
                file_path,
                String::from_utf8_lossy(&info_log)
            ));
        }

        println!("{} shader was compiled successfully.", shader_type);
        Ok(Shader {
            gl: gl.clone(),
            id: shader_id,
        })
    }

    pub fn from_vertex_source(
        gl: &gl::Gl,
        res: &ResourceLoader,
        file_path: &str,
    ) -> Result<Shader, String> {
        Shader::from_source(gl, res, file_path, gl::VERTEX_SHADER)
    }

    pub fn from_fragment_source(
        gl: &gl::Gl,
        res: &ResourceLoader,
        file_path: &str,
    ) -> Result<Shader, String> {
        Shader::from_source(gl, res, file_path, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}
