mod camera;
mod material;
mod mesh;
mod model;
mod resources;
mod shader;
mod skybox;
mod texture;
mod uniform;
mod uniform_buffer_object;

use std::path::Path;

use cgmath::{Array, Deg, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix};
use glfw::{Context, OpenGlProfileHint, WindowHint};

use camera::{Camera, CameraController};

use resources::ResourceLoader;
use shader::{Program, Shader};
use skybox::Skybox;
use uniform::{Uniform, Uniform3f};
use uniform_buffer_object::UniformBufferObject;

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

    let camera_controller = CameraController::new(5.0, 0.15);
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

    // --- TEMP ---
    let model_3d = resources.load_model(&gl, "assets/models/stone_cube/scene.gltf", &program);
    // ------------

    // GLOBAL UNIFORMS
    let matrix4_size = std::mem::size_of::<Matrix4<f32>>() as isize;

    let matrix_ubo = UniformBufferObject::new(
        &gl,
        0,
        &[
            ("projection", matrix4_size),
            ("view", matrix4_size),
            ("model", matrix4_size),
            ("normalMatrix", matrix4_size),
        ],
    );

    let light_ubo = UniformBufferObject::new(
        &gl,
        1,
        &[("lightPos", std::mem::size_of::<Point3<f32>>() as isize)],
    );

    // SKYBOX
    let skybox = Skybox::new(&gl, &resources);

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

            // SET GLOBAL UNIFORMS
            let model_matrix = Matrix4::<f32>::from_angle_x(cgmath::Deg(-90.0));
            let normal_matrix = model_matrix.invert().unwrap().transpose();

            matrix_ubo.write_sub_data("projection", camera.get_projection().as_ptr().cast());
            matrix_ubo.write_sub_data("view", camera.get_view().as_ptr().cast());
            matrix_ubo.write_sub_data("model", model_matrix.as_ptr().cast());
            matrix_ubo.write_sub_data("normalMatrix", normal_matrix.as_ptr().cast());

            let light_pos = Point3::<f32>::new(-1.5, 1.5, 1.5);
            light_ubo.write_sub_data("lightPos", light_pos.as_ptr().cast());

            // SKYBOX
            skybox.draw(&camera);

            let uniforms: Vec<Box<dyn Uniform>> = vec![
                // Uniform3f::new("lightPos", light_pos),
                Uniform3f::new("viewPos", camera.get_position()),
            ];

            model_3d.draw(uniforms);
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
