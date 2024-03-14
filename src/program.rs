use anyhow::{bail, Result};

use crate::shader::Shader;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()) };
        }

        unsafe { gl::LinkProgram(program_id) };

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()) };
        }

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetShaderiv(program_id, gl::LINK_STATUS, &mut success) };

        if success == 0 {
            let mut log_len = 0_i32;
            let mut info_log: Vec<u8> = Vec::with_capacity(1024);

            unsafe {
                gl::GetProgramInfoLog(program_id, 512, &mut log_len, info_log.as_mut_ptr().cast());
                info_log.set_len(log_len.try_into().unwrap());
            }

            bail!(
                "Error: Program linking failed: {}",
                String::from_utf8_lossy(&info_log)
            );
        };

        Ok(Program { id: program_id })
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) };
    }
}
