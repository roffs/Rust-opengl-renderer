use crate::{
    mesh::{
        vertex::{MeshVertex, Vertex},
        Mesh,
    },
    texture::Texture,
};

static CUBE_VERTICES: [MeshVertex; 24] = [
    // Front
    MeshVertex::new((-0.5, -0.5, 0.5), (0.0, 0.0)),
    MeshVertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
    MeshVertex::new((0.5, 0.5, 0.5), (1.0, 1.0)),
    MeshVertex::new((-0.5, 0.5, 0.5), (0.0, 1.0)),
    // Back
    MeshVertex::new((-0.5, -0.5, -0.5), (0.0, 0.0)),
    MeshVertex::new((0.5, -0.5, -0.5), (1.0, 0.0)),
    MeshVertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
    MeshVertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
    // Left
    MeshVertex::new((-0.5, -0.5, -0.5), (0.0, 0.0)),
    MeshVertex::new((-0.5, -0.5, 0.5), (1.0, 0.0)),
    MeshVertex::new((-0.5, 0.5, 0.5), (1.0, 1.0)),
    MeshVertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
    // Right
    MeshVertex::new((0.5, -0.5, -0.5), (0.0, 0.0)),
    MeshVertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
    MeshVertex::new((0.5, 0.5, 0.5), (1.0, 1.0)),
    MeshVertex::new((0.5, 0.5, -0.5), (0.0, 1.0)),
    // Top
    MeshVertex::new((-0.5, 0.5, 0.5), (0.0, 0.0)),
    MeshVertex::new((0.5, 0.5, 0.5), (1.0, 0.0)),
    MeshVertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
    MeshVertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
    // Bottom
    MeshVertex::new((-0.5, -0.5, 0.5), (0.0, 0.0)),
    MeshVertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
    MeshVertex::new((0.5, -0.5, -0.5), (1.0, 1.0)),
    MeshVertex::new((-0.5, -0.5, -0.5), (0.0, 1.0)),
];

const CUBE_INDICES: [i32; 36] = [
    0, 1, 2, 0, 2, 3, // Front
    4, 7, 6, 4, 6, 5, // Back
    8, 9, 10, 8, 10, 11, // Left
    12, 15, 14, 12, 14, 13, // Right
    16, 17, 18, 16, 18, 19, // Top
    20, 23, 22, 20, 22, 21, // Bottom
];

impl<'a> Mesh<'a, MeshVertex> {
    pub fn create_cube(gl: &gl::Gl, texture: Texture) -> Mesh<'a, MeshVertex> {
        Mesh::create(gl, &CUBE_VERTICES, &CUBE_INDICES, texture)
    }
}
