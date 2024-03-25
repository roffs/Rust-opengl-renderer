use crate::{
    material::Material,
    mesh::{Mesh, MeshVertex},
};

pub struct Model<'a> {
    meshes: Vec<(Mesh<MeshVertex>, i32)>,
    materials: Vec<Material<'a>>,
}

impl<'a> Model<'a> {
    pub fn new(meshes: Vec<(Mesh<MeshVertex>, i32)>, materials: Vec<Material>) -> Model {
        Model { meshes, materials }
    }

    pub fn draw(&self, uniforms: &[(&str, cgmath::Matrix4<f32>)]) {
        for (mesh, material_index) in &self.meshes {
            let material = self.materials.get(*material_index as usize).unwrap();
            material.use_program();
            material.set_uniforms(uniforms);
            mesh.draw(material);
        }
    }
}
