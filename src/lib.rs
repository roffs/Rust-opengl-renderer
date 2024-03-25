mod camera;
mod material;
mod mesh;
mod model;
mod resources;
mod shader;
mod texture;

use std::path::Path;

use cgmath::{Matrix, Matrix4};
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

    // INIT RESOURCES LOADER

    let resources = ResourceLoader::from_relative_exe_path(Path::new("")).unwrap();

    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);

    let camera_controller = CameraController::new(10.0);
    let mut camera = Camera::new(
        (0.0, 3.0, 3.0),
        (0.0, -1.0, -1.0),
        (0.0, 1.0, 0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
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

    let mut movement_direction = cgmath::vec3(0.0, 0.0, 0.0);
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

            model_3d.draw(&[("model", model_matrix)]);
        }

        glfw.poll_events();
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) => match key {
                    glfw::Key::W => movement_direction.z -= 0.1,
                    glfw::Key::A => movement_direction.x -= 0.1,
                    glfw::Key::S => movement_direction.z += 0.1,
                    glfw::Key::D => movement_direction.x += 0.1,
                    glfw::Key::Space => movement_direction.y += 0.1,
                    glfw::Key::LeftShift => movement_direction.y -= 0.1,
                    _ => {}
                },
                glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) => match key {
                    glfw::Key::W => movement_direction.z += 0.1,
                    glfw::Key::A => movement_direction.x += 0.1,
                    glfw::Key::S => movement_direction.z -= 0.1,
                    glfw::Key::D => movement_direction.x -= 0.1,
                    glfw::Key::Space => movement_direction.y -= 0.1,
                    glfw::Key::LeftShift => movement_direction.y += 0.1,
                    _ => {}
                },
                _ => {}
            }
        }
        camera_controller.translate(&mut camera, movement_direction * delta_time);
    }
}
