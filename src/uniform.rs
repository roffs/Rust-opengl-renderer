use cgmath::Matrix;

pub trait Uniform {
    fn set(&self, gl: &gl::Gl, program: u32);
}

pub struct UniformMat4f<'a> {
    name: &'a str,
    value: cgmath::Matrix4<f32>,
}

impl UniformMat4f<'_> {
    pub fn new(name: &str, value: cgmath::Matrix4<f32>) -> Box<UniformMat4f> {
        Box::new(UniformMat4f { name, value })
    }
}

impl<'a> Uniform for UniformMat4f<'a> {
    fn set(&self, gl: &gl::Gl, program_id: u32) {
        let location = get_location(gl, program_id, self.name).unwrap();
        unsafe { gl.UniformMatrix4fv(location, 1, gl::FALSE, self.value.as_ptr().cast()) };
    }
}

pub struct Uniform3f<'a> {
    name: &'a str,
    value: (f32, f32, f32),
}

impl Uniform3f<'_> {
    pub fn new<T: Into<(f32, f32, f32)>>(name: &str, value: T) -> Box<Uniform3f> {
        Box::new(Uniform3f {
            name,
            value: value.into(),
        })
    }
}
impl<'a> Uniform for Uniform3f<'a> {
    fn set(&self, gl: &gl::Gl, program_id: u32) {
        let location = get_location(gl, program_id, self.name).unwrap();
        let (v0, v1, v2) = self.value;
        unsafe { gl.Uniform3f(location, v0, v1, v2) };
    }
}

pub struct UniformInt<'a> {
    name: &'a str,
    value: i32,
}

impl UniformInt<'_> {
    pub fn new(name: &str, value: i32) -> Box<UniformInt> {
        Box::new(UniformInt { name, value })
    }
}
impl<'a> Uniform for UniformInt<'a> {
    fn set(&self, gl: &gl::Gl, program_id: u32) {
        let location = get_location(gl, program_id, self.name).unwrap();
        unsafe { gl.Uniform1i(location, self.value) };
    }
}

fn get_location(
    gl: &gl::Gl,
    program_id: u32,
    uniform_name: &str,
) -> Result<gl::types::GLint, String> {
    let uniform_cname =
        std::ffi::CString::new(uniform_name).expect("expected uniform name to have no nul bytes");

    let location = unsafe { gl.GetUniformLocation(program_id, uniform_cname.as_ptr().cast()) };

    match location {
        -1 => Err(format!(
            "Uniform location \"{}\" was not found in program with id {}",
            uniform_name, program_id
        )),
        _ => Ok(location),
    }
}
