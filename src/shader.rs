use anyhow::Result;
use core::panic;
use std::{fs, sync::Arc};

use glow::{Context, HasContext, NativeShader, Program};

pub struct Shader {
    program: Program,
    gl: Arc<Context>,
}

impl Shader {
    pub fn new(
        gl: Arc<Context>,
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Self> {
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

            gl.link_program(program);

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { program, gl })
        }
    }
    pub fn use_program(&self) {
        unsafe { self.gl.use_program(Some(self.program)) };
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let location = self.gl.get_uniform_location(self.program, name);
            self.gl.uniform_1_i32(location.as_ref(), value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let location = self.gl.get_uniform_location(self.program, name);
            self.gl.uniform_1_i32(location.as_ref(), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let location = self.gl.get_uniform_location(self.program, name);
            self.gl.uniform_1_f32(location.as_ref(), value);
        }
    }

    pub fn set_mat4(&self, name: &str, value: &glm::Mat4) {
        unsafe {
            let location = self.gl.get_uniform_location(self.program, name);
            self.gl
                .uniform_matrix_4_f32_slice(location.as_ref(), false, value.as_slice());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
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
