mod camera_controller;

use cgmath::{perspective, Angle, Deg, Matrix4, Point3, Rad, Vector3};

pub use camera_controller::CameraController;

pub struct Camera {
    pub(self) position: Point3<f32>,
    pub(self) yaw: Rad<f32>,
    pub(self) pitch: Rad<f32>,
    up: Vector3<f32>,
    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new<T: Into<cgmath::Rad<f32>>>(
        position: (f32, f32, f32),

        yaw: T,
        pitch: T,

        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera {
            position: Point3::from(position),
            yaw: yaw.into(),
            pitch: pitch.into(),
            up: Vector3::new(0.0, 1.0, 0.0),
            fovy,
            aspect,
            near,
            far,
        }
    }

    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        let x = self.yaw.cos() * self.pitch.cos();
        let y = self.pitch.sin();
        let z = self.yaw.sin() * self.pitch.cos();
        Matrix4::look_to_rh(self.position, Vector3::from((x, y, z)), self.up)
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }
}
