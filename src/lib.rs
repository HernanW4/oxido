use anyhow::Result;
use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use log;

use app::App;
use util::create_window_attrs;

mod app;
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
    event_loop.run_app(&mut app)?;
    Ok(())
}
