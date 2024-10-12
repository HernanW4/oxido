use std::{collections::HashMap, sync::Arc};

use crate::camera::Camera;
use glow::{Context, HasContext};
use glutin::prelude::GlDisplay;
use winit::event::WindowEvent;

use crate::{
    mesh::{Mesh, Vertex},
    shader::Shader,
};

pub struct Renderer {
    shader: Shader,
    gl: Arc<Context>,
    meshes: Vec<Mesh>,

    camera: Camera,
    aspect_ratio: f32,
}

impl Renderer {
    pub fn new<D: GlDisplay>(
        gl_display: &D,
        vertex_source_path: &str,
        fragment_source_path: &str,
    ) -> Self {
        unsafe {
            let gl = Arc::new(glow::Context::from_loader_function_cstr(|s| {
                gl_display.get_proc_address(s)
            }));

            let shader = Shader::new(gl.clone(), vertex_source_path, fragment_source_path)
                .expect("Could not make Shader");

            let camera = Camera::new(
                glm::vec3(0.0, 0.0, 3.0),
                glm::vec3(0.0, 1.0, 0.0),
                -90.0,
                0.0,
            );

            Renderer {
                shader,
                gl,
                meshes: Vec::new(),
                camera,
                aspect_ratio: 1.0,
            }
        }
    }

    pub fn add_mesh(&mut self, mesh_data: (Vec<Vertex>, Vec<u32>)) {
        let (vertices, indices) = mesh_data;

        let mesh = Mesh::new(self.gl.clone(), vertices, indices);

        self.meshes.push(mesh);
    }

    pub fn draw_default(&self) {
        self.draw_with_clear_color(0.1, 0.2, 0.3, 0.9);
    }

    pub fn draw_with_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            self.gl.clear_color(red, green, blue, alpha);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            self.shader.use_program();

            let view = self.camera.get_view_matrix();
            let projection = glm::perspective(self.aspect_ratio, 45.0f32.to_radians(), 0.1, 100.0);

            self.shader.set_mat4("view", &view);
            self.shader.set_mat4("projection", &projection);

            self.meshes.iter().for_each(|m| m.draw(&self.shader));
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }

    pub fn process_input(&mut self, camera_speed: f32, event: WindowEvent) {
        self.camera.process_input(camera_speed, event);
    }
}
