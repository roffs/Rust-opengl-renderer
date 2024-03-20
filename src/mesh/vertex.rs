use cgmath::*;

pub trait Vertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl);
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MeshVertex {
    position: Point3<f32>,
    uv: Vector2<f32>,
}

impl MeshVertex {
    pub const fn new(position: (f32, f32, f32), uv: (f32, f32)) -> MeshVertex {
        MeshVertex {
            position: Point3::new(position.0, position.1, position.2),
            uv: Vector2::new(uv.0, uv.1),
        }
    }
}

impl Vertex for MeshVertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl) {
        unsafe {
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl.EnableVertexAttribArray(1);
            gl.EnableVertexAttribArray(0);
        }
    }
}
