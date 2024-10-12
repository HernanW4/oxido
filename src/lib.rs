use anyhow::Result;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use log;

extern crate nalgebra_glm as glm;

use app::App;
use mesh::{Mesh, Vertex};
use util::create_window_attrs;

mod app;
mod camera;
mod mesh;
mod renderer;
mod shader;
mod util;

pub fn run() -> Result<()> {
    log::debug!("Hey");
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(create_window_attrs()));

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = App::new(template, display_builder);

    setup_mesh(&mut app);

    event_loop.run_app(&mut app)?;
    Ok(())
}

fn setup_mesh(app: &mut App) {
    // Create a triangle mesh
    let triangle_vertices = vec![
        Vertex {
            position: glm::vec3(0.5, -0.5, 0.0),
            normals: glm::vec3(0.0, 0.0, 1.0),
        },
        Vertex {
            position: glm::vec3(-0.5, -0.5, 0.0),
            normals: glm::vec3(0.0, 0.0, 1.0),
        },
        Vertex {
            position: glm::vec3(0.0, 0.5, 0.0),
            normals: glm::vec3(0.0, 0.0, 1.0),
        },
    ];

    let indices = vec![0, 1, 2];
    // Note: We can't create the Mesh here because we don't have the GL context yet.
    // Instead, we'll store the vertex and index data and create the Mesh later.
    app.add_mesh(triangle_vertices, indices);
}
