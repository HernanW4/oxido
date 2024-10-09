use anyhow::Result;
use core::panic;
use std::fs;

use glow::{Context, HasContext, NativeShader, Program};
use glutin::prelude::GlDisplay;

pub struct Shader {
    program: Program,
}

impl Shader {
    pub fn new(gl: &Context, vertex_shader_path: &str, fragment_shader_path: &str) -> Result<Self> {
        unsafe {
            let (vertex_shader, fragment_shader) = {
                let vertex_source = fs::read_to_string(vertex_shader_path)?;
                let fragment_source = fs::read_to_string(fragment_shader_path)?;

                (
                    create_shader(&gl, glow::VERTEX_SHADER, &vertex_source),
                    create_shader(&gl, glow::FRAGMENT_SHADER, &fragment_source),
                )
            };

            let program = gl.create_program().expect("Cannot create a program");

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);

            gl.link_program(program);

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            Ok(Self { program })
        }
    }
    pub fn use_program(&self, gl: &Context) {
        unsafe { gl.use_program(Some(self.program)) };
    }
}

pub struct Renderer {
    shader: Shader,
    gl: Context,
    vao: glow::VertexArray,
}

impl Renderer {
    pub fn new<D: GlDisplay>(
        gl_display: &D,
        vertex_source_path: &str,
        fragment_source_path: &str,
    ) -> Self {
        unsafe {
            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));

            let shader = Shader::new(&gl, vertex_source_path, fragment_source_path)
                .expect("Could not make Shader");

            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            gl.bind_vertex_array(Some(vao));

            Renderer { shader, gl, vao }
        }
    }

    pub fn draw_default(&self) {
        self.draw_with_clear_color(0.1, 0.2, 0.3, 1.0);
    }

    pub fn draw_with_clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            self.shader.use_program(&self.gl);

            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.clear_color(red, green, blue, alpha);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(0, 0, width, height);
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
