use glfw::{Context, OpenGlProfileHint, WindowHint};

fn main() {
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

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        process_input(&mut window);

        glfw.poll_events();
        window.swap_buffers();
    }
}

fn resize_callback(_: &mut glfw::Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height) };
}

fn process_input(window: &mut glfw::Window) {
    if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
        window.set_should_close(true);
    }
}
