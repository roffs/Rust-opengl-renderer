use std::os::raw::c_void;

pub struct UniformBufferObject<'a> {
    gl: gl::Gl,
    id: u32,
    size: isize,
    sub_uniforms: Vec<(&'a str, isize)>,
}

impl<'a> UniformBufferObject<'a> {
    pub fn new(
        gl: &gl::Gl,
        binding: u32,
        sub_uniforms: &[(&'a str, isize)],
    ) -> UniformBufferObject<'a> {
        let mut id = 0;

        let total_size = sub_uniforms.iter().map(|(_, size)| size).sum();

        unsafe {
            gl.GenBuffers(1, &mut id);
            gl.BindBuffer(gl::UNIFORM_BUFFER, id);
            gl.BufferData(
                gl::UNIFORM_BUFFER,
                total_size,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );

            gl.BindBufferRange(gl::UNIFORM_BUFFER, binding, id, 0, total_size)
        };

        UniformBufferObject {
            gl: gl.clone(),
            id,
            size: total_size,
            sub_uniforms: sub_uniforms.to_vec(),
        }
    }

    pub fn write_sub_data(&self, uniform: &str, data: *const c_void) {
        let mut offset = 0;

        let (_, size) = self
            .sub_uniforms
            .iter()
            .find(|(name, size)| {
                if *name != uniform {
                    offset += size;
                    false
                } else {
                    true
                }
            })
            .unwrap();

        unsafe {
            self.gl.BindBuffer(gl::UNIFORM_BUFFER, self.id);
            self.gl
                .BufferSubData(gl::UNIFORM_BUFFER, offset, *size, data)
        };
    }
}
