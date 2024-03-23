use crate::{
    material::Material,
    mesh::{Mesh, MeshVertex},
};

pub struct Model {
    meshes: Vec<(Mesh<MeshVertex>, i32)>,
    materials: Vec<Material>,
}

impl Model {
    pub fn new(meshes: Vec<(Mesh<MeshVertex>, i32)>, materials: Vec<Material>) -> Model {
        Model { meshes, materials }
    }

    pub fn draw(&self) {
        for (mesh, material_index) in &self.meshes {
            mesh.draw(self.materials.get(*material_index as usize).unwrap());
        }
    }
}
