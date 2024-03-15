mod program;
mod shader;
mod texture;
mod vertex;

use anyhow::Result;

use glfw::{Context, OpenGlProfileHint, WindowHint};
use program::Program;
use shader::Shader;
use texture::Texture;
use vertex::Vertex;

fn main() -> Result<()> {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            1080,
            720,
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

    // VAO & VBO

    let vertices: [Vertex; 3] = [
        Vertex::new((-0.5, -0.5, 0.0), (0.0, 0.0)),
        Vertex::new((0.5, -0.5, 0.0), (1.0, 0.0)),
        Vertex::new((0.0, 0.5, 0.0), (0.5, 1.0)),
    ];

    let mut vertex_array_object = 0;
    unsafe { gl.GenVertexArrays(1, &mut vertex_array_object) };

    let mut vertex_buffer = 0;
    unsafe { gl.GenBuffers(1, &mut vertex_buffer) };

    unsafe {
        gl.BindVertexArray(vertex_array_object);
        gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<Vertex>() * vertices.len()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl.VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl.VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * std::mem::size_of::<f32>() as i32,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl.EnableVertexAttribArray(1);

        gl.EnableVertexAttribArray(0);
    }

    // SET UNIFORMS

    let uniform_color_location = shader_program.get_uniform_location("ourColor")?;
    shader_program.set_uniform_4f(uniform_color_location, (0.0, 1.0, 0.0, 1.0));

    // TEXTURE

    let texture = Texture::load(&gl, "src/textures/texture.png")?;
    texture.bind();

    // EVENT LOOP

    while !window.should_close() {
        unsafe {
            gl.ClearColor(0.3, 0.4, 0.6, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);

            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        process_input(&mut window);

        glfw.poll_events();
        window.swap_buffers();

        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = event {
                window.set_should_close(true);
            }
        }
    }

    Ok(())
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}
