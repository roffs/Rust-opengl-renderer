use cgmath::Deg;

use super::Camera;

pub struct CameraController {
    move_speed: f32,
    rotation_speed: f32,
}

impl CameraController {
    pub fn new(move_speed: f32, rotation_speed: f32) -> CameraController {
        CameraController {
            move_speed,
            rotation_speed,
        }
    }

    pub fn translate(&self, camera: &mut Camera, direction: cgmath::Vector3<f32>) {
        camera.position += direction * self.move_speed;
    }

    pub fn rotate(&self, camera: &mut Camera, delta: (f32, f32)) {
        let (yaw, pitch) = delta;

        camera.yaw += Deg(yaw * self.rotation_speed).into();
        camera.pitch -= Deg(pitch * self.rotation_speed).into();
    }
}
