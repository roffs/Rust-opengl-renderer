mod camera_controller;

use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3};

pub use camera_controller::CameraController;

pub struct Camera {
    pub position: Point3<f32>,
    direction: Vector3<f32>,
    up: Vector3<f32>,
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
            position: Point3::from(position),
            direction: Vector3::from(direction).normalize(),
            up: Vector3::from(up).normalize(),
            fovy,
            aspect,
            near,
            far,
        }
    }

    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.direction, self.up)
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }
}
