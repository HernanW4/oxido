use anyhow::Result;
use component::Entity;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use graphics::{mesh::MeshData, vertex::Vertex};
use log;

extern crate nalgebra_glm as glm;

use app::App;
use util::create_window_attrs;

mod app;
mod camera;
mod chunk;
mod component;
mod graphics;
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

    setup_entities(&mut app);

    event_loop.run_app(&mut app)?;
    Ok(())
}

fn setup_entities(app: &mut App) {
    // Note: We can't create the Mesh here because we don't have the GL context yet.
    // Instead, we'll store the vertex and index data and create the Mesh later.
    //
    //
    let scene = app.get_scene();

    let mesh_data = create_cube_mesh_data();

    let entity = Entity::new().with_mesh_data(mesh_data);

    scene.add_entity(entity);
}

fn create_cube_mesh_data() -> MeshData {
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
    let vertices: Vec<Vertex> = cube_vertices
        .chunks(3)
        .enumerate()
        .map(|(_i, v)| Vertex {
            position: glm::vec3(v[0], v[1], v[2]),
            normals: glm::Vec3::zeros(),
            colors: glm::vec3(0.0, 1.0, 0.0),
        })
        .collect();

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

    MeshData::new(vertices, indices)
}
