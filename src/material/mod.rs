use crate::{
    shader::Program,
    texture::Texture,
    uniform::{Uniform, UniformInt},
};

pub struct Material<'a> {
    program: &'a Program,
    base_color: Texture,
    normal: Texture,
}

impl<'a> Material<'a> {
    pub fn new(program: &Program, base_color: Texture, normal: Texture) -> Material {
        Material {
            program,
            base_color,
            normal,
        }
    }

    pub fn use_material(&self, uniforms: &Vec<Box<dyn Uniform>>) {
        self.program.use_program();
        self.set_textures();
        self.program.set_uniforms(uniforms)
    }

    fn set_textures(&self) {
        self.base_color.bind(gl::TEXTURE0);
        self.normal.bind(gl::TEXTURE1);

        let uniforms: Vec<Box<dyn Uniform>> = vec![
            UniformInt::new("diffuseTexture", 0),
            UniformInt::new("normalTexture", 1),
        ];

        self.program.set_uniforms(&uniforms);
    }
}
