use cgmath::{vec3, Vector3};

#[repr(C, packed)]
pub struct Vertex {
    position: Vector3<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: vec3(x, y, z),
        }
    }
}
