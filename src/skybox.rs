use cgmath::Point3;

use crate::{
    camera::Camera,
    mesh::{Mesh, Vertex},
    resources::ResourceLoader,
    shader::{Program, Shader},
    texture::CubeMap,
    uniform::UniformMat4f,
};

pub struct Skybox {
    gl: gl::Gl,
    mesh: Mesh,
    cube_map: CubeMap,
    program: Program,
}

impl Skybox {
    pub fn new(gl: &gl::Gl, res: &ResourceLoader) -> Skybox {
        let vertices = vec![
            // Front
            SkyboxVertex::new((-1.0, -1.0, 1.0)),
            SkyboxVertex::new((1.0, -1.0, 1.0)),
            SkyboxVertex::new((1.0, 1.0, 1.0)),
            SkyboxVertex::new((-1.0, 1.0, 1.0)),
            // Back
            SkyboxVertex::new((-1.0, -1.0, -1.0)),
            SkyboxVertex::new((1.0, -1.0, -1.0)),
            SkyboxVertex::new((1.0, 1.0, -1.0)),
            SkyboxVertex::new((-1.0, 1.0, -1.0)),
            // Left
            SkyboxVertex::new((-1.0, -1.0, -1.0)),
            SkyboxVertex::new((-1.0, -1.0, 1.0)),
            SkyboxVertex::new((-1.0, 1.0, 1.0)),
            SkyboxVertex::new((-1.0, 1.0, -1.0)),
            // Right
            SkyboxVertex::new((1.0, -1.0, -1.0)),
            SkyboxVertex::new((1.0, -1.0, 1.0)),
            SkyboxVertex::new((1.0, 1.0, 1.0)),
            SkyboxVertex::new((1.0, 1.0, -1.0)),
            // Top
            SkyboxVertex::new((-1.0, 1.0, 1.0)),
            SkyboxVertex::new((1.0, 1.0, 1.0)),
            SkyboxVertex::new((1.0, 1.0, -1.0)),
            SkyboxVertex::new((-1.0, 1.0, -1.0)),
            // Bottom
            SkyboxVertex::new((-1.0, -1.0, 1.0)),
            SkyboxVertex::new((1.0, -1.0, 1.0)),
            SkyboxVertex::new((1.0, -1.0, -1.0)),
            SkyboxVertex::new((-1.0, -1.0, -1.0)),
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3, // Front
            4, 7, 6, 4, 6, 5, // Back
            8, 9, 10, 8, 10, 11, // Left
            12, 15, 14, 12, 14, 13, // Right
            16, 17, 18, 16, 18, 19, // Top
            20, 23, 22, 20, 22, 21, // Bottom
        ];
        let mesh = Mesh::create(gl, vertices, indices);

        let images_paths = [
            "assets/skybox/sky/right.jpg",
            "assets/skybox/sky/left.jpg",
            "assets/skybox/sky/top.jpg",
            "assets/skybox/sky/bottom.jpg",
            "assets/skybox/sky/front.jpg",
            "assets/skybox/sky/back.jpg",
        ];
        let cube_map = CubeMap::load(gl, res, images_paths).unwrap();

        let vertex_shader =
            Shader::from_vertex_source(gl, res, "assets/shaders/skybox.vert").unwrap();
        let fragment_shader =
            Shader::from_fragment_source(gl, res, "assets/shaders/skybox.frag").unwrap();
        let program = Program::from_shaders(gl, &[vertex_shader, fragment_shader]).unwrap();

        Skybox {
            gl: gl.clone(),
            mesh,
            cube_map,
            program,
        }
    }

    pub fn draw(&self, camera: &Camera) {
        unsafe {
            self.gl.DepthMask(gl::FALSE);
            self.program.use_program();

            self.program
                .set_uniforms(&vec![UniformMat4f::new("view", camera.get_rotation())]);
            self.gl.BindTexture(gl::TEXTURE_CUBE_MAP, self.cube_map.id);

            self.mesh.draw();

            self.gl.DepthMask(gl::TRUE);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct SkyboxVertex {
    position: Point3<f32>,
}

impl SkyboxVertex {
    pub fn new<T: Into<cgmath::Point3<f32>>>(position: T) -> SkyboxVertex {
        SkyboxVertex {
            position: position.into(),
        }
    }
}

impl Vertex for SkyboxVertex {
    fn set_vertex_attrib_pointer(gl: &gl::Gl) {
        let stride = 3 * std::mem::size_of::<f32>() as i32;

        unsafe {
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl.EnableVertexAttribArray(0);
        }
    }
}
