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

impl Mesh<MeshVertex> {
    pub fn create_cube(gl: &gl::Gl, texture: Texture) -> Mesh<MeshVertex> {
        let vertices = CUBE_VERTICES;
        let indices = CUBE_INDICES;

        let mut vao = 0;
        unsafe { gl.GenVertexArrays(1, &mut vao) };

        let mut vbo = 0;
        unsafe { gl.GenBuffers(1, &mut vbo) };

        let mut ebo = 0;
        unsafe { gl.GenBuffers(1, &mut ebo) };

        unsafe {
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<MeshVertex>() * vertices.len()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<i32>() * indices.len()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            MeshVertex::set_vertex_attrib_pointer(gl);

            gl.BindVertexArray(0);
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Mesh {
            gl: gl.clone(),

            vertices,
            indices,
            texture,

            vao,
            vbo,
            ebo,
        }
    }
}
