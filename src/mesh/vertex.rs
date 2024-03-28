use cgmath::*;

pub trait Vertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl);
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MeshVertex {
    position: Point3<f32>,
    uv: Vector2<f32>,
    normal: Vector3<f32>,
    tangent: Vector3<f32>,
    bitangent: Vector3<f32>,
}

impl MeshVertex {
    pub fn new<
        T: Into<cgmath::Point3<f32>>,
        U: Into<cgmath::Vector3<f32>>,
        V: Into<cgmath::Vector2<f32>>,
    >(
        position: T,
        uv: V,
        normal: U,
        tangent: U,
        bitangent: U,
    ) -> MeshVertex {
        MeshVertex {
            position: position.into(),
            uv: uv.into(),
            normal: normal.into(),
            tangent: tangent.into(),
            bitangent: bitangent.into(),
        }
    }
}

impl Vertex for MeshVertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl) {
        let stride = 14 * std::mem::size_of::<f32>() as i32;

        unsafe {
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl.VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl.VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * std::mem::size_of::<f32>()) as *const _,
            );
            gl.VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (8 * std::mem::size_of::<f32>()) as *const _,
            );
            gl.VertexAttribPointer(
                4,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (11 * std::mem::size_of::<f32>()) as *const _,
            );

            gl.EnableVertexAttribArray(0);
            gl.EnableVertexAttribArray(1);
            gl.EnableVertexAttribArray(2);
            gl.EnableVertexAttribArray(3);
            gl.EnableVertexAttribArray(4);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SkyboxVertex {
    position: Point3<f32>,
}

impl SkyboxVertex {
    pub fn new<T: Into<cgmath::Point3<f32>>>(position: T) -> SkyboxVertex {
        SkyboxVertex {
            position: position.into(),
        }
    }
}

impl Vertex for SkyboxVertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl) {
        let stride = 3 * std::mem::size_of::<f32>() as i32;

        println!("setting skybox vertex attrib pointer");
        unsafe {
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl.EnableVertexAttribArray(0);
        }
    }
}
