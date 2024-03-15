use cgmath::{perspective, Deg, Matrix4};

pub struct Camera {
    pub position: (f32, f32, f32),
    direction: (f32, f32, f32),
    up: (f32, f32, f32),
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(
        position: (f32, f32, f32),
        direction: (f32, f32, f32),
        up: (f32, f32, f32),
        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera {
            position,
            direction,
            up,
            fovy,
            aspect,
            near,
            far,
        }
    }

    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh(self.position.into(), self.direction.into(), self.up.into())
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }
}
