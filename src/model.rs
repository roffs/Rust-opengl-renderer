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

    pub fn draw(&self, global_uniforms: &[(&str, cgmath::Matrix4<f32>)]) {
        for (mesh, material_index) in &self.meshes {
            let material = self.materials.get(*material_index as usize).unwrap();
            material.use_program();
            material.set_global_uniforms(global_uniforms);
            mesh.draw(material);
        }
    }
}
