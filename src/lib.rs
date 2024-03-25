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

use camera::Camera;

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

    while !window.should_close() {
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

            // let model = Matrix4::from_translation((-2.5, 0.0, 0.0).into());
            // let uniform_model_location = shader_program.get_uniform_location("model").unwrap();
            // shader_program.set_uniform_matrix_4fv(uniform_model_location, model);
            // // cube.draw();
        }

        glfw.poll_events();
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(glfw::Key::A, _, glfw::Action::Press, _) => {
                    camera.position.x -= 0.1;
                }
                glfw::WindowEvent::Key(glfw::Key::D, _, glfw::Action::Press, _) => {
                    camera.position.x += 0.1;
                }
                glfw::WindowEvent::Key(glfw::Key::W, _, glfw::Action::Press, _) => {
                    camera.position.y += 0.1;
                }
                glfw::WindowEvent::Key(glfw::Key::S, _, glfw::Action::Press, _) => {
                    camera.position.y -= 0.1;
                }
                _ => {}
            }
        }
    }
}
