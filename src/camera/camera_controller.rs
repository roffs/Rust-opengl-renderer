use super::Camera;

pub struct CameraController {
    move_speed: f32,
}

impl CameraController {
    pub fn new(move_speed: f32) -> CameraController {
        CameraController { move_speed }
    }

    pub fn translate(&self, camera: &mut Camera, direction: cgmath::Vector3<f32>) {
        camera.position += direction * self.move_speed;
    }
}
