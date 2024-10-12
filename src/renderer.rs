use std::{collections::HashMap, sync::Arc};

use crate::{
    camera::Camera,
    scene::{Object, Scene},
};
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

            gl.enable(glow::CCW);

            let shader = Shader::new(gl.clone(), vertex_source_path, fragment_source_path)
                .expect("Could not make Shader");

            Renderer { shader, gl }
        }
    }

    pub fn draw_default(&self) {
        self.draw_with_clear_color(0.1, 0.2, 0.3, 0.9);
    }

    pub fn render(&self, scene: &mut Scene) {
        let (view, projection) = scene.get_camera_attributes();
        self.shader.use_program();
        self.shader.set_mat4("view", &view);
        self.shader.set_mat4("projection", &projection);

        for object in scene.get_objects().iter_mut() {
            let model = object.get_transformation();
            let mesh = object.mesh();

            mesh.setup_mesh(self.gl.clone());

            self.shader.set_mat4("model", &model);

            unsafe {
                self.gl.bind_vertex_array(Some(mesh.vao()));
                self.gl.draw_elements(
                    glow::TRIANGLES,
                    mesh.indices.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                );

                self.gl.bind_vertex_array(None);
            }
        }
    }

    pub fn draw_with_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            self.gl.clear_color(red, green, blue, alpha);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        log::debug!("Resizing to: {width} {height}");
        unsafe {
            self.gl.viewport(0, 0, width, height);
        }
    }
}
