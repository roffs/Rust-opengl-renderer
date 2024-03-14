use cgmath::*;

#[repr(C, packed)]
pub struct Vertex {
    position: Vector3<f32>,
    uv: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: (f32, f32, f32), uv: (f32, f32)) -> Vertex {
        Vertex {
            position: position.into(),
            uv: uv.into(),
        }
    }
}
