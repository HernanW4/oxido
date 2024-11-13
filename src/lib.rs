use anyhow::Result;
use entity::Entity;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use log;

extern crate nalgebra_glm as glm;

use app::App;
use util::create_window_attrs;

mod app;
mod camera;
mod component;
mod entity;
mod graphics;
mod renderer;
mod scene;
mod shader;
mod util;

pub fn run() -> Result<()> {
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(create_window_attrs()));

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = App::new(template, display_builder);

    setup_entities(&mut app);

    log::info!("Started event loop");
    event_loop.run_app(&mut app)?;
    log::info!("Finished event loop");
    Ok(())
}

fn setup_entities(app: &mut App) {
    // Note: We can't create the Mesh here because we don't have the GL context yet.
    // Instead, we'll store the vertex and index data and create the Mesh later.
    //
    //
    let scene = app.get_scene();

    for i in 0..=5 {
        let x = i as f32;
        let entity = Entity::cube_entity().with_position(glm::vec3(x, 0.0, 0.0));
        scene.add_entity(entity);
    }
}
