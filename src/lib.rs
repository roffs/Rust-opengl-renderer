mod camera;
mod material;
mod mesh;
mod model;
mod resources;
mod shader;
mod texture;

use std::path::Path;

use cgmath::{Deg, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix, Vector3};
use glfw::{Context, OpenGlProfileHint, WindowHint};

use camera::{Camera, CameraController};

use resources::ResourceLoader;
use shader::{Program, Shader};

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

pub fn run() {
    // INITIALIZE GRAPHICS AND WINDOW CONTEXT

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            WIDTH,
            HEIGHT,
            "OpenGL Rust Renderer",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // INIT RESOURCES LOADER

    let resources = ResourceLoader::from_relative_exe_path(Path::new("")).unwrap();

    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);

    let camera_controller = CameraController::new(10.0, 0.2);
    let mut camera = Camera::new(
        (0.0, 0.0, 3.0),
        Deg(-90.0),
        Deg(0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
        0.01,
        100.0,
    );

    // CREATE SHADER PROGRAM
    let vertex_shader =
        Shader::from_vertex_source(&gl, &resources, "assets/shaders/shader.vert").unwrap();
    let fragment_shader =
        Shader::from_fragment_source(&gl, &resources, "assets/shaders/shader.frag").unwrap();
    let program = Program::from_shaders(&gl, &[vertex_shader, fragment_shader]).unwrap();

    program.set_global_uniforms();

    // --- TEMP ---
    let model_3d = resources.load_model(&gl, "assets/models/stone_cube/scene.gltf", &program);
    // ------------

    // GLOBAL UNIFORMS
    let mut ubo = 0;
    unsafe {
        gl.GenBuffers(1, &mut ubo);
        gl.BindBuffer(gl::UNIFORM_BUFFER, ubo);
        gl.BufferData(
            gl::UNIFORM_BUFFER,
            2 * std::mem::size_of::<cgmath::Matrix4<f32>>() as isize,
            std::ptr::null(),
            gl::STATIC_DRAW,
        );
        gl.BindBufferRange(
            gl::UNIFORM_BUFFER,
            0,
            ubo,
            0,
            2 * std::mem::size_of::<cgmath::Matrix4<f32>>() as isize,
        );
    }

    // ENABLE DEPTH TESTING

    unsafe { gl.Enable(gl::DEPTH_TEST) };

    // EVENT LOOP

    // x respresents forward movement, y side movement and z vertical movement
    let mut movement_direction = cgmath::vec3(0.0, 0.0, 0.0);
    let mut initial_mouse_pos = window.get_cursor_pos();

    let mut initial_time = 0.0;

    while !window.should_close() {
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - initial_time;
        initial_time = current_time;

        unsafe {
            gl.ClearColor(0.3, 0.4, 0.6, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl.BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                std::mem::size_of::<cgmath::Matrix4<f32>>() as isize,
                camera.get_projection().as_ptr().cast(),
            );

            gl.BufferSubData(
                gl::UNIFORM_BUFFER,
                std::mem::size_of::<cgmath::Matrix4<f32>>() as isize,
                std::mem::size_of::<cgmath::Matrix4<f32>>() as isize,
                camera.get_view().as_ptr().cast(),
            );

            let model_matrix = Matrix4::from_angle_x(cgmath::Deg(-90.0));
            let normal_matrix = model_matrix.invert().unwrap().transpose();

            let light_pos = Point3::new(-1.5, 1.5, 1.5);

            model_3d.draw(
                &[("model", model_matrix), ("normalMatrix", normal_matrix)],
                &[("lightPos", light_pos), ("viewPos", camera.get_position())],
            );
        }

        glfw.poll_events();
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => match (key, action) {
                    (glfw::Key::Escape, glfw::Action::Press) => window.set_should_close(true),
                    (key, glfw::Action::Press) => match key {
                        glfw::Key::W => movement_direction.x += 1.0,
                        glfw::Key::A => movement_direction.z -= 1.0,
                        glfw::Key::S => movement_direction.x -= 1.0,
                        glfw::Key::D => movement_direction.z += 1.0,
                        glfw::Key::Space => movement_direction.y += 1.0,
                        glfw::Key::LeftShift => movement_direction.y -= 1.0,
                        _ => {}
                    },
                    (key, glfw::Action::Release) => match key {
                        glfw::Key::W => movement_direction.x -= 1.0,
                        glfw::Key::A => movement_direction.z += 1.0,
                        glfw::Key::S => movement_direction.x += 1.0,
                        glfw::Key::D => movement_direction.z -= 1.0,
                        glfw::Key::Space => movement_direction.y -= 1.0,
                        glfw::Key::LeftShift => movement_direction.y += 1.0,
                        _ => {}
                    },
                    _ => {}
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (x_diff, y_diff) = (x - initial_mouse_pos.0, y - initial_mouse_pos.1);
                    camera_controller.rotate(&mut camera, (x_diff as f32, y_diff as f32));
                    initial_mouse_pos = (x, y);
                }
                _ => {}
            }
        }

        let normalized_movement_direction = match movement_direction.magnitude2() > 0.0 {
            true => movement_direction.normalize(),
            false => movement_direction,
        };

        camera_controller.translate(&mut camera, normalized_movement_direction * delta_time);
    }
}
