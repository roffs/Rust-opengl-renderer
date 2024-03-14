mod program;
mod shader;
mod vertex;

use anyhow::Result;

use glfw::{Context, OpenGlProfileHint, WindowHint};
use program::Program;
use shader::Shader;
use vertex::Vertex;

fn main() -> Result<()> {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, _events) = glfw
        .create_window(
            1080,
            720,
            "OpenGL Rust Renderer",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    window.set_framebuffer_size_callback(resize_callback);

    // SHADER PROGRAM

    let vertex_shader = Shader::from_vertex_source("src/shaders/shader.vert")?;
    let fragment_shader = Shader::from_fragment_source("src/shaders/shader.frag")?;

    let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader])?;
    shader_program.use_program();

    // VAO & VBO

    let vertices: [Vertex; 3] = [
        Vertex::new(-0.5, -0.5, 0.0),
        Vertex::new(0.5, -0.5, 0.0),
        Vertex::new(0.0, 0.5, 0.0),
    ];

    let mut vertex_array_object = 0;
    unsafe { gl::GenVertexArrays(1, &mut vertex_array_object) };

    let mut vertex_buffer = 0;
    unsafe { gl::GenBuffers(1, &mut vertex_buffer) };

    unsafe {
        gl::BindVertexArray(vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<Vertex>() * vertices.len()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);
    }

    // EVENT LOOP

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        process_input(&mut window);

        glfw.poll_events();
        window.swap_buffers();
    }

    Ok(())
}

fn resize_callback(_: &mut glfw::Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) };
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}
