mod primitives;
mod vertex;

use crate::texture::Texture;

use self::vertex::Vertex;

pub struct Mesh<T: Vertex> {
    gl: gl::Gl,

    vertices: Vec<T>,
    indices: Vec<i32>,
    texture: Texture,

    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
}

impl<T: Vertex> Mesh<T> {
    pub fn create(gl: &gl::Gl, vertices: Vec<T>, indices: Vec<i32>, texture: Texture) -> Mesh<T> {
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
                (std::mem::size_of::<T>() * vertices.len()) as isize,
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

            T::set_vertex_attrib_pointer(gl);
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

    pub fn draw(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.texture.bind(gl::TEXTURE0);

            self.gl.DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        };
    }
}
