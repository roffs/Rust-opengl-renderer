mod camera;
mod cube;
mod program;
mod shader;
mod texture;
mod vertex;

use cgmath::{Matrix4, Vector3};
use cube::Cube;
use glfw::{Context, OpenGlProfileHint, WindowHint};

use camera::Camera;
use program::Program;
use shader::Shader;
use texture::Texture;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;

pub fn run() -> anyhow::Result<()> {
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

    let gl = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);

    // SHADER PROGRAM

    let vertex_shader = Shader::from_vertex_source(&gl, "src/shaders/shader.vert")?;
    let fragment_shader = Shader::from_fragment_source(&gl, "src/shaders/shader.frag")?;

    let shader_program = Program::from_shaders(&gl, &[vertex_shader, fragment_shader])?;
    shader_program.use_program();

    // SET UNIFORMS

    let uniform_color_location = shader_program.get_uniform_location("ourColor")?;
    shader_program.set_uniform_4f(uniform_color_location, (0.0, 1.0, 0.0, 1.0));

    let model = Matrix4::from_translation(Vector3::new(0.2, 0.2, 0.2));
    let uniform_model_location = shader_program.get_uniform_location("model")?;
    shader_program.set_uniform_matrix_4fv(uniform_model_location, model);

    let mut camera = Camera::new(
        (0.0, 3.0, 3.0),
        (0.0, -1.0, -1.0),
        (0.0, 1.0, 0.0),
        45.0,
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        100.0,
    );

    // CUBE

    let texture = Texture::load(&gl, "src/textures/texture.png")?;
    let cube = Cube::new(&gl, texture);

    // ENABLE DEPTH TESTING

    unsafe { gl.Enable(gl::DEPTH_TEST) };

    // EVENT LOOP

    while !window.should_close() {
        let uniform_view_location = shader_program.get_uniform_location("view")?;
        shader_program.set_uniform_matrix_4fv(uniform_view_location, camera.get_view());

        let uniform_projection_location = shader_program.get_uniform_location("projection")?;
        shader_program.set_uniform_matrix_4fv(uniform_projection_location, camera.get_projection());

        unsafe {
            gl.ClearColor(0.3, 0.4, 0.6, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            cube.draw();
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

    Ok(())
}
