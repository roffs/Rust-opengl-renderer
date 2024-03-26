use cgmath::{Angle, Rad, Vector3};

pub(super) fn calculate_local_directions(
    yaw: Rad<f32>,
    pitch: Rad<f32>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();

    let look_dir = Vector3::new(x, y, z);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = look_dir.cross(up);
    let forward = up.cross(right);

    (look_dir, up, right, forward)
}
