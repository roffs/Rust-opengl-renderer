use crate::texture::Texture;

pub struct Material {
    base_color: Texture,
}

impl Material {
    pub fn new(base_color: Texture) -> Material {
        Material { base_color }
    }

    pub fn bind(&self) {
        self.base_color.bind(gl::TEXTURE0);
    }
}
