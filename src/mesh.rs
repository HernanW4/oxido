use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext};

use crate::shader::Shader;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normals: glm::Vec3,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct VertexPod {
    position: [f32; 3],
    normals: [f32; 3],
}

impl From<Vertex> for VertexPod {
    fn from(value: Vertex) -> Self {
        let position: [f32; 3] = value.position.into();
        let normals: [f32; 3] = value.normals.into();

        VertexPod { position, normals }
    }
}

pub struct Mesh {
    vao: glow::VertexArray,
    gl: Arc<Context>,
    vbo: glow::Buffer,
    ebo: glow::Buffer,

    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(gl: Arc<Context>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let (vao, vbo, ebo) = Mesh::setup_mesh(gl.clone(), vertices.clone(), indices.clone());
        Self {
            gl,
            vao,
            vbo,
            ebo,
            vertices,
            indices,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        unsafe {
            let model = glm::Mat4::new_scaling(0.5);

            shader.set_mat4("model", &model);

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_elements(
                glow::TRIANGLES,
                self.indices.len() as i32,
                glow::UNSIGNED_INT,
                0,
            );
            self.gl.bind_vertex_array(None);
        }
    }

    fn setup_mesh(
        gl: Arc<Context>,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> (glow::VertexArray, glow::Buffer, glow::Buffer) {
        unsafe {
            let vao = gl.create_vertex_array().expect("Could not create VAO");
            let vbo = gl.create_buffer().expect("Could not create buffer for VBO");
            let ebo = gl.create_buffer().expect("Could not create buffer for EBO");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertex_pod: Vec<VertexPod> = vertices.iter().map(|v| VertexPod::from(*v)).collect();

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertex_pod),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                glow::STATIC_DRAW,
            );

            //Vertex Pos
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            //Vertex Normals
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                std::mem::offset_of!(Vertex, normals) as i32,
            );
            gl.enable_vertex_attrib_array(1);

            gl.bind_vertex_array(None);

            (vao, vbo, ebo)
        }
    }
}
