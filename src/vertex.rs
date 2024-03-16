use cgmath::*;

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Vertex {
    position: Point3<f32>,
    uv: Vector2<f32>,
}

impl Vertex {
    pub const fn new(position: (f32, f32, f32), uv: (f32, f32)) -> Vertex {
        Vertex {
            position: Point3::new(position.0, position.1, position.2),
            uv: Vector2::new(uv.0, uv.1),
        }
    }
}
