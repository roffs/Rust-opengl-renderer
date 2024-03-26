use cgmath::Matrix;

use crate::shader::Shader;

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
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

            return Err(format!(
                "Error: Program linking failed: {}",
                String::from_utf8_lossy(&info_log)
            ));
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

    pub fn set_global_uniforms(&self) {
        unsafe {
            let uniform_block_index = self
                .gl
                .GetUniformBlockIndex(self.id, "Matrices".as_bytes().as_ptr() as *const i8);
            self.gl.UniformBlockBinding(self.id, uniform_block_index, 0);
        };
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<gl::types::GLint, String> {
        let uniform_cname =
            std::ffi::CString::new(name).expect("expected uniform name to have no nul bytes");

        let location = unsafe {
            self.gl
                .GetUniformLocation(self.id, uniform_cname.as_ptr().cast())
        };

        match location {
            -1 => Err(format!(
                "Uniform location \"{}\" was not found in program with id {}",
                name, self.id
            )),
            _ => Ok(location),
        }
    }

    pub fn set_uniform_4f(&self, name: &str, value: (f32, f32, f32, f32)) {
        unsafe {
            let location = self.get_uniform_location(name).unwrap();
            self.gl
                .Uniform4f(location, value.0, value.1, value.2, value.3)
        };
    }

    pub fn set_uniform_3f(&self, name: &str, value: (f32, f32, f32)) {
        unsafe {
            let location = self.get_uniform_location(name).unwrap();
            self.gl.Uniform3f(location, value.0, value.1, value.2)
        };
    }

    pub fn set_uniform_matrix_4fv(&self, name: &str, value: cgmath::Matrix4<f32>) {
        unsafe {
            let location = self.get_uniform_location(name).unwrap();
            self.gl
                .UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr().cast())
        };
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let location = self.get_uniform_location(name).unwrap();
            self.gl.Uniform1i(location, value)
        };
    }
}
