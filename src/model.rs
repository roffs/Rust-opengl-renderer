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

    // TODO: Improve how we handle the uniforms here
    pub fn draw<T: Into<(f32, f32, f32)> + Copy>(
        &self,
        mat4f_uniforms: &[(&str, cgmath::Matrix4<f32>)],
        vec3f_uniforms: &[(&str, T)],
    ) {
        for (mesh, material_index) in &self.meshes {
            let material = self.materials.get(*material_index as usize).unwrap();
            material.use_program();
            material.set_mat4f_uniforms(mat4f_uniforms);
            material.set_vec3f_uniforms(vec3f_uniforms);
            mesh.draw(material);
        }
    }
}
