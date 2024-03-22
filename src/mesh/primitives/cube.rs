use crate::mesh::{vertex::MeshVertex, Mesh};

impl Mesh<MeshVertex> {
    pub fn create_cube(gl: &gl::Gl) -> Mesh<MeshVertex> {
        let cube_vertices = [
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

        let cube_indices = [
            0, 1, 2, 0, 2, 3, // Front
            4, 7, 6, 4, 6, 5, // Back
            8, 9, 10, 8, 10, 11, // Left
            12, 15, 14, 12, 14, 13, // Right
            16, 17, 18, 16, 18, 19, // Top
            20, 23, 22, 20, 22, 21, // Bottom
        ];

        Mesh::create(gl, cube_vertices.into(), cube_indices.into())
    }
}
