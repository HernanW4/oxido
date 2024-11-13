use anyhow::Result;
use core::panic;
use std::fs;

use glow::{Context, HasContext, NativeShader, Program};

use crate::graphics::gl::GlResource;

pub struct Shader {
    program: Program,
    gl_resource: GlResource,
}

impl Shader {
    pub fn new(
        gl_resource: GlResource,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Self> {
        log::debug!("Using vertex shader: {vertex_shader_path}");
        log::debug!("Using fragment shader: {fragment_shader_path}");
        let gl = gl_resource.gl();
        unsafe {
            let (vertex_shader, fragment_shader) = {
                let vertex_source = fs::read_to_string(vertex_shader_path)?;
                let fragment_source = fs::read_to_string(fragment_shader_path)?;

                (
                    create_shader(&gl, glow::VERTEX_SHADER, &vertex_source),
                    create_shader(&gl, glow::FRAGMENT_SHADER, &fragment_source),
                )
            };

            let program = gl.create_program().expect("Could not create program");

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            log::info!("Shaders attached!");

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                log::error!("Link program error: {}", gl.get_program_info_log(program));
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self {
                program,
                gl_resource,
            })
        }
    }
    pub fn use_program(&self) {
        let gl = self.gl_resource.gl();
        unsafe { gl.use_program(Some(self.program)) };
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let gl = self.gl_resource.gl();
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let gl = self.gl_resource.gl();
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_i32(location.as_ref(), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let gl = self.gl_resource.gl();
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_1_f32(location.as_ref(), value);
        }
    }

    pub fn set_mat4(&self, name: &str, value: &glm::Mat4) {
        let gl = self.gl_resource.gl();
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, value.as_slice());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        let gl = self.gl_resource.gl();
        unsafe {
            gl.delete_program(self.program);
        }
    }
}

fn create_shader(gl: &Context, shader_type: u32, shader_source: &str) -> NativeShader {
    unsafe {
        let shader = gl.create_shader(shader_type).expect("Cannot create shader");

        gl.shader_source(shader, shader_source);

        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            panic!("{}", gl.get_shader_info_log(shader));
        }

        shader
    }
}
