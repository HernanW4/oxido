use anyhow::Result;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use log;

extern crate nalgebra_glm as glm;

use app::App;
use mesh::{Mesh, Vertex};
use scene::Object;
use util::create_window_attrs;

mod app;
mod camera;
mod mesh;
mod renderer;
mod scene;
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
    let cube_vertices = vec![
        -0.5, -0.5, 0.5, // Front face
        0.5, -0.5, 0.5, //
        0.5, 0.5, 0.5, //
        -0.5, 0.5, 0.5, //
        -0.5, -0.5, -0.5, // Back face
        0.5, -0.5, -0.5, //
        0.5, 0.5, -0.5, //
        -0.5, 0.5, -0.5, //
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

    // Note: We can't create the Mesh here because we don't have the GL context yet.
    // Instead, we'll store the vertex and index data and create the Mesh later.
    //

    for x in 0..=6 {
        let mut cube = Object::new(cube_vertices.clone(), indices.clone());
        let new_pos = glm::vec3(x as f32, 1.0, -1.0);
        cube.set_pos(new_pos);
        app.add_objects(cube);
    }
}
