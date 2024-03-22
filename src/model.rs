use crate::{
    mesh::{Mesh, MeshVertex},
    texture::Texture,
};

pub struct Model {
    meshes: Vec<(Mesh<MeshVertex>, i32)>,
    textures: Vec<Texture>,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh<MeshVertex>, i32)>, textures: Vec<Texture>) -> Model {
        Model { meshes, textures }
    }

    pub fn draw(&self) {
        for (mesh, texture_index) in &self.meshes {
            mesh.draw(self.textures.get(*texture_index as usize).unwrap());
        }
    }
}
