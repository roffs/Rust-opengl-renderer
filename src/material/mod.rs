use crate::{shader::Program, texture::Texture};

pub struct Material {
    program: Program,
    base_color: Texture,
    normal: Texture,
}

impl Material {
    pub fn new(program: Program, base_color: Texture, normal: Texture) -> Material {
        Material {
            program,
            base_color,
            normal,
        }
    }

    pub fn bind(&self) {
        self.base_color.bind(gl::TEXTURE0);
        self.program.set_int("diffuse", 0);
        self.normal.bind(gl::TEXTURE1);
        self.program.set_int("normal", 1);
    }

    pub fn set_global_uniforms(&self, uniforms: &[(&str, cgmath::Matrix4<f32>)]) {
        for (name, value) in uniforms {
            self.program.set_uniform_matrix_4fv(name, *value);
        }
    }

    pub fn use_program(&self) {
        self.program.use_program();
    }
}
