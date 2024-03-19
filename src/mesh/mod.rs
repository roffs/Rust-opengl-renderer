mod primitives;
mod vertex;

use crate::texture::Texture;

use self::vertex::Vertex;

pub struct Mesh {
    gl: gl::Gl,

    vertices: [Vertex; 24],
    indices: [i32; 36],
    texture: Texture,

    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
}

impl Mesh {
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

            self.gl.BindVertexArray(0);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            self.texture.unbind();
        };
    }
}
