use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use glow::{Context, HasContext};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normals: glm::Vec3,
    pub colors: glm::Vec3,
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct VertexPod {
    position: [f32; 3],
    normals: [f32; 3],
    colors: [f32; 3],
}

impl From<Vertex> for VertexPod {
    fn from(value: Vertex) -> Self {
        let position: [f32; 3] = value.position.into();
        let normals: [f32; 3] = value.normals.into();
        let colors: [f32; 3] = value.colors.into();

        VertexPod {
            position,
            normals,
            colors,
        }
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,
    ebo: Option<glow::Buffer>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vao: None,
            vbo: None,
            ebo: None,
        }
    }

    pub fn vao(&mut self) -> glow::VertexArray {
        return self.vao.take().expect("VAO was not initalized");
    }

    pub fn setup_mesh(&mut self, gl: Arc<Context>) {
        unsafe {
            let vao = gl.create_vertex_array().expect("Could not create VAO");
            let vbo = gl.create_buffer().expect("Could not create buffer for VBO");
            let ebo = gl.create_buffer().expect("Could not create buffer for EBO");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertex_pod: Vec<VertexPod> =
                self.vertices.iter().map(|v| VertexPod::from(*v)).collect();

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertex_pod),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&self.indices),
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

            //Vertex Color
            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                std::mem::offset_of!(Vertex, colors) as i32,
            );
            gl.enable_vertex_attrib_array(2);

            gl.bind_vertex_array(None);

            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.ebo = Some(ebo);
        }
    }
}
