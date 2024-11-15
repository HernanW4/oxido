use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use glow::HasContext;

use crate::graphics::vertex::Vertex;
use log;

use super::mesh::{MeshData, RenderableMesh};

#[derive(Debug)]
pub enum GlowModes {
    EnableCulling,
    DisableCulling,
    LinesOnly,
    FillOnly,
    CullFill,
    CullLine,
}
//Converts glm::vec3 to [f32;3]
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct VertexPod {
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

#[derive(Debug, Clone)]
pub struct GlResource {
    gl: Arc<glow::Context>,
}

impl GlResource {
    pub fn new(gl: glow::Context) -> Self {
        Self { gl: Arc::new(gl) }
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }
}

pub trait Graphics {
    fn create_renderable_mesh(&self, mesh_data: &MeshData) -> RenderableMesh;
    fn delete_renderable_mesh(&self, mesh: &RenderableMesh);
    fn draw_mesh(&self, mesh: &RenderableMesh);
    fn clear_with_color(&self, red: f32, green: f32, blue: f32);
    fn resize(&self, width: f32, height: f32);
    fn drawing_mode(&self);
    fn change_drawing_mode(&mut self, mode: GlowModes);
}

pub struct GlGraphics {
    gl_resource: GlResource,
    mode: GlowModes,
}

impl GlGraphics {
    pub fn new(gl_resource: GlResource) -> Self {
        Self {
            gl_resource,
            mode: GlowModes::CullLine,
        }
    }
}

impl Graphics for GlGraphics {
    fn create_renderable_mesh(&self, mesh_data: &MeshData) -> RenderableMesh {
        let gl = self.gl_resource.gl();
        unsafe {
            let vao = gl.create_vertex_array().expect("Could not create VAO");
            let vbo = gl.create_buffer().expect("Could not create buffer for VBO");
            let ebo = gl.create_buffer().expect("Could not create buffer for EBO");

            gl.bind_vertex_array(Some(vao));
            //log::debug!("VAO has been binded");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            //log::debug!("VBO has been binded");

            let vertex_pod: Vec<VertexPod> = mesh_data
                .vertices()
                .iter()
                .map(|v| VertexPod::from(*v))
                .collect();

            assert!(
                vertex_pod.len() == mesh_data.vertices().len(),
                "The vector of VertexPod is not the same as the vector of received Vertex"
            );

            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertex_pod),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            //log::debug!("EBO has been binded");
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&mesh_data.indices()),
                glow::STATIC_DRAW,
            );

            //Vertex Pos
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<VertexPod>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            //Vertex Normals
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<VertexPod>() as i32,
                std::mem::offset_of!(VertexPod, normals) as i32,
            );
            gl.enable_vertex_attrib_array(1);

            //Vertex Color
            gl.vertex_attrib_pointer_f32(
                2,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<VertexPod>() as i32,
                std::mem::offset_of!(VertexPod, colors) as i32,
            );
            gl.enable_vertex_attrib_array(2);

            gl.bind_vertex_array(None);

            //log::info!("RenderableMesh has been created");
            RenderableMesh::new(vao, vbo, ebo, mesh_data.indices().len() as u32)
        }
    }
    fn delete_renderable_mesh(&self, mesh: &RenderableMesh) {
        let gl = self.gl_resource.gl();
        let (vao, vbo, ebo) = mesh.mesh_arrays_buffers();

        unsafe {
            gl.delete_vertex_array(*vao);
            gl.delete_buffer(*vbo);
            gl.delete_buffer(*ebo);
        }
    }
    fn draw_mesh(&self, mesh: &RenderableMesh) {
        let gl = self.gl_resource.gl();
        let (vao, ..) = mesh.mesh_arrays_buffers();

        unsafe {
            gl.bind_vertex_array(Some(*vao));
            gl.draw_elements(
                glow::TRIANGLES,
                mesh.index_count() as i32,
                glow::UNSIGNED_INT,
                0,
            );
            gl.bind_vertex_array(None);
        }
    }

    fn clear_with_color(&self, red: f32, green: f32, blue: f32) {
        let gl = self.gl_resource.gl();

        unsafe {
            gl.clear_color(red, green, blue, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
        }
    }

    fn resize(&self, width: f32, height: f32) {
        let (width, height) = (width as i32, height as i32);
        log::debug!("Resizing to: {width} {height}");
        let gl = self.gl_resource.gl();
        unsafe {
            gl.viewport(0, 0, width, height);
        }
    }

    fn change_drawing_mode(&mut self, mode: GlowModes) {
        self.mode = mode;
    }

    fn drawing_mode(&self) {
        let gl = self.gl_resource.gl();

        unsafe {
            match self.mode {
                GlowModes::EnableCulling => {
                    gl.enable(glow::CULL_FACE);
                    gl.front_face(glow::BACK);
                }

                GlowModes::DisableCulling => {
                    gl.disable(glow::CULL_FACE);
                }
                GlowModes::CullLine => {
                    gl.enable(glow::CULL_FACE);
                    gl.front_face(glow::CCW);
                    gl.cull_face(glow::BACK);
                    gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
                }
                GlowModes::LinesOnly => {
                    gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
                }
                GlowModes::FillOnly => {
                    gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
                }
                GlowModes::CullFill => {
                    gl.enable(glow::CULL_FACE);
                    gl.front_face(glow::CCW);
                    gl.cull_face(glow::BACK);
                    gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
                }
                _ => {}
            }
        }
    }
}
