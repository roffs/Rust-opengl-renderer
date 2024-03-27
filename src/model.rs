use crate::{
    material::Material,
    mesh::{Mesh, MeshVertex},
    uniform::Uniform,
};

pub struct Model<'a> {
    meshes: Vec<(Mesh<MeshVertex>, i32)>,
    materials: Vec<Material<'a>>,
}

impl<'a> Model<'a> {
    pub fn new(meshes: Vec<(Mesh<MeshVertex>, i32)>, materials: Vec<Material>) -> Model {
        Model { meshes, materials }
    }

    // TODO: Improve how we handle the uniforms here
    pub fn draw(&self, uniforms: Vec<Box<dyn Uniform>>) {
        for (mesh, material_index) in &self.meshes {
            let material = self.materials.get(*material_index as usize).unwrap();
            material.use_material(&uniforms);
            mesh.draw();
        }
    }
}
