use std::fs;

use anyhow::{bail, Result};

use glfw::{Context, OpenGlProfileHint, WindowHint};

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

    let vertex_shader = create_shader("src/shaders/shader.vert", gl::VERTEX_SHADER)?;
    let fragment_shader = create_shader("src/shaders/shader.frag", gl::FRAGMENT_SHADER)?;

    let shader_program = create_shader_program(vertex_shader, fragment_shader)?;

    unsafe { gl::UseProgram(shader_program) };

    // VAO & VBO

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vertex_array_object = 0;
    unsafe { gl::GenVertexArrays(1, &mut vertex_array_object) };

    let mut vertex_buffer = 0;
    unsafe { gl::GenBuffers(1, &mut vertex_buffer) };

    unsafe {
        gl::BindVertexArray(vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as isize,
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

fn create_shader(file_path: &str, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint> {
    let shader_source =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let shader = unsafe { gl::CreateShader(shader_type) };

    let shader_type = match shader_type {
        gl::VERTEX_SHADER => "Vertex",
        gl::FRAGMENT_SHADER => "Fragment",
        _ => "Unspecified type",
    };

    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &shader_source.as_bytes().as_ptr().cast(),
            std::ptr::null(),
        );
        gl::CompileShader(shader);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut log_len = 0_i32;
        let mut info_log: Vec<u8> = Vec::with_capacity(1024);

        unsafe {
            gl::GetShaderInfoLog(shader, 512, &mut log_len, info_log.as_mut_ptr().cast());
            info_log.set_len(log_len.try_into().unwrap());
        }

        bail!(
            "Error: {} shader from {} compilation failed: {}",
            shader_type,
            file_path,
            String::from_utf8_lossy(&info_log)
        );
    }

    println!("{} shader was compiled successfully.", shader_type);
    Ok(shader)
}

fn create_shader_program(
    vertex_shader: gl::types::GLenum,
    fragment_shader: gl::types::GLenum,
) -> Result<gl::types::GLuint> {
    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(program, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut log_len = 0_i32;
        let mut info_log: Vec<u8> = Vec::with_capacity(1024);

        unsafe {
            gl::GetProgramInfoLog(program, 512, &mut log_len, info_log.as_mut_ptr().cast());
            info_log.set_len(log_len.try_into().unwrap());
        }

        bail!(
            "Error: Program linking failed: {}",
            String::from_utf8_lossy(&info_log)
        );
    };

    println!("Shader program was linked successfully");
    Ok(program)
}
