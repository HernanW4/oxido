use glutin::{
    config::{Config, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentContext},
    display::GetGlDisplay,
    prelude::GlDisplay,
};
use raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes};

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

pub fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
            major: 4,
            minor: 1,
        })))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &legacy_context_attributes)
                    .expect("failed to create context")
            })
    }
}

pub fn create_window_attrs() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("Hello")
}

pub mod predetermined_etentities {
    use crate::entity::Entity;
    use crate::graphics::mesh::MeshData;

    pub fn create_cube() -> Entity {
        use crate::graphics::vertex::Vertex;
        // Create a triangle mesh
        //let cube_vertices = vec![
        //    -0.5, -0.5, 0.5, // Front face
        //    0.5, -0.5, 0.5, //
        //    0.5, 0.5, 0.5, //
        //    -0.5, 0.5, 0.5, //
        //    -0.5, -0.5, -0.5, // Back face
        //    0.5, -0.5, -0.5, //
        //    0.5, 0.5, -0.5, //
        //    -0.5, 0.5, -0.5, //
        //];
        //let vertices: Vec<Vertex> = cube_vertices
        //    .chunks(3)
        //    .enumerate()
        //    .map(|(i, v)| {
        //        let is_edge = i % 4 == 0 || i % 4 == 1;

        //        Vertex {
        //            position: glm::vec3(v[0], v[1], v[2]),
        //            normals: glm::Vec3::zeros(),
        //            colors: if is_edge {
        //                glm::vec3(0.0, 0.8, 0.0)
        //            } else {
        //                glm::vec3(0.0, 1.0, 0.0)
        //            },
        //        }
        //    })
        //    .collect();

        let vertices = vec![
            // Front face vertices
            Vertex {
                position: glm::vec3(-0.5, -0.5, 0.5),
                normals: glm::vec3(0.0, 0.0, 1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(0.5, -0.5, 0.5),
                normals: glm::vec3(0.0, 0.0, 1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(0.5, 0.5, 0.5),
                normals: glm::vec3(0.0, 0.0, 1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(-0.5, 0.5, 0.5),
                normals: glm::vec3(0.0, 0.0, 1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            // Back face vertices
            Vertex {
                position: glm::vec3(-0.5, -0.5, -0.5),
                normals: glm::vec3(0.0, 0.0, -1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(0.5, -0.5, -0.5),
                normals: glm::vec3(0.0, 0.0, -1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(0.5, 0.5, -0.5),
                normals: glm::vec3(0.0, 0.0, -1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: glm::vec3(-0.5, 0.5, -0.5),
                normals: glm::vec3(0.0, 0.0, -1.0),
                colors: glm::vec3(0.0, 1.0, 0.0),
            },
        ];
        let indices = vec![
            // Front face
            0, 1, 2, //
            2, 3, 0, //
            // Back face
            4, 5, 6, //
            6, 7, 4, //
            // Left face
            4, 7, 3, //
            3, 0, 4, //
            // Right face
            1, 5, 6, //
            6, 2, 1, //
            // Top face
            3, 2, 6, //
            6, 7, 3, //
            // Bottom face
            0, 1, 5, //
            5, 4, 0, //
        ];

        let mesh_data = MeshData::new(vertices, indices);

        Entity::new().with_mesh_data(mesh_data)
    }
}
