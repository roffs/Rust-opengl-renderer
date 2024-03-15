use anyhow::{bail, Result};

use crate::shader::Shader;

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()) };
        }

        unsafe { gl.LinkProgram(program_id) };

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()) };
        }

        let mut success: gl::types::GLint = 1;
        unsafe { gl.GetShaderiv(program_id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut log_len = 0_i32;
            let mut info_log: Vec<u8> = Vec::with_capacity(1024);

            unsafe {
                gl.GetProgramInfoLog(program_id, 512, &mut log_len, info_log.as_mut_ptr().cast());
                info_log.set_len(log_len.try_into().unwrap());
            }

            bail!(
                "Error: Program linking failed: {}",
                String::from_utf8_lossy(&info_log)
            );
        };

        println!("Shader program was created successfully");
        Ok(Program {
            gl: gl.clone(),
            id: program_id,
        })
    }

    pub fn use_program(&self) {
        unsafe { self.gl.UseProgram(self.id) };
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<gl::types::GLint> {
        let uniform_cname =
            std::ffi::CString::new(name).expect("expected uniform name to have no nul bytes");

        let location = unsafe {
            self.gl
                .GetUniformLocation(self.id, uniform_cname.as_ptr().cast())
        };

        if location == -1 {
            bail!(
                "Uniform location \"{}\" was not found in program with id {}",
                name,
                self.id
            )
        }

        Ok(location)
    }

    pub fn set_uniform_4f(&self, location: gl::types::GLint, value: (f32, f32, f32, f32)) {
        unsafe {
            self.gl
                .Uniform4f(location, value.0, value.1, value.2, value.3)
        };
    }
}
