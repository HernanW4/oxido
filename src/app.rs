use std::{borrow::Borrow, num::NonZeroU32, time::Instant};

use glutin::{
    config::{ConfigTemplateBuilder, GetGlConfig, GlConfig},
    context::PossiblyCurrentContext,
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, SwapInterval, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, KeyCode, NamedKey},
    window::Window,
};

use crate::{
    graphics::gl::GlResource,
    renderer::{DrawingMode, Renderer},
    util::{create_gl_context, create_window_attrs},
};
use crate::{scene::Scene, util::gl_config_picker};

const VERTEX_PATH: &'static str = "shaders/vertex.glsl";
const FRAGMENT_PATH: &'static str = "shaders/fragment.glsl";

pub struct App {
    state: Option<AppState>,
    template: ConfigTemplateBuilder,
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayState,

    renderer: Option<Renderer>,
    scene: Scene,
    last_updated: Instant,
}

impl App {
    pub fn new(template: ConfigTemplateBuilder, display_builder: DisplayBuilder) -> Self {
        Self {
            template,
            gl_display: GlDisplayState::Build(display_builder),
            state: None,
            gl_context: None,
            renderer: None,
            scene: Scene::new(glm::vec3(1.0, 1.0, -1.0)),
            last_updated: Instant::now(),
        }
    }

    pub fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let (window, gl_config) = match &self.gl_display {
            GlDisplayState::Build(display_builder) => {
                let (window, gl_config) = match display_builder.clone().build(
                    event_loop,
                    self.template.clone(),
                    gl_config_picker,
                ) {
                    Ok((window, gl_config)) => (window.unwrap(), gl_config),
                    Err(err) => {
                        log::error!("{:?}", err);
                        event_loop.exit();
                        return;
                    }
                };
                log::info!("Picked a config with {} samples", gl_config.num_samples());

                self.gl_display = GlDisplayState::Init;

                self.gl_context =
                    Some(create_gl_context(&window, &gl_config).treat_as_possibly_current());

                (window, gl_config)
            }
            GlDisplayState::Init => {
                log::info!("Recreating window in resumed");

                let gl_config = self.gl_context.as_ref().unwrap().config();
                match glutin_winit::finalize_window(event_loop, create_window_attrs(), &gl_config) {
                    Ok(window) => (window, gl_config),
                    Err(err) => {
                        log::error!("{:?}", err);
                        event_loop.exit();
                        return;
                    }
                }
            }
        };

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface).unwrap();

        unsafe {
            let gl = glow::Context::from_loader_function_cstr(|s| {
                gl_context.display().get_proc_address(s)
            });

            let gl_resource = GlResource::new(gl);

            self.renderer
                .get_or_insert_with(|| Renderer::new(gl_resource, (VERTEX_PATH, FRAGMENT_PATH)));
        }
        // Try setting vsync.
        if let Err(res) = gl_surface
            .set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }

        assert!(self
            .state
            .replace(AppState { gl_surface, window })
            .is_none());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                // Some platforms like EGL require resizing GL surface to update the size
                // Notable platforms here are Wayland and macOS, other don't require it
                // and the function is no-op, but it's wise to resize it for portability
                // reasons.
                if let Some(AppState {
                    gl_surface,
                    window: _,
                }) = self.state.as_ref()
                {
                    let gl_context = self.gl_context.as_ref().unwrap();
                    gl_surface.resize(
                        gl_context,
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    );

                    let renderer = self.renderer.as_ref().unwrap();
                    renderer.resize(size.width as f32, size.height as f32);
                }
            }
            WindowEvent::Focused(focused) => {
                if let Some(AppState { window, .. }) = &self.state {
                    if focused {
                        //window.unwrap();
                        window.set_cursor_visible(false);
                    }
                }
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                let renderer = self.renderer.as_mut().unwrap();
                event_loop.exit();
                renderer.after_closing();
            }
            WindowEvent::RedrawRequested => {}
            ev => {
                self.scene.process_input(ev);
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // NOTE: The handling below is only needed due to nvidia on Wayland to not crash
        // on exit due to nvidia driver touching the Wayland display from on
        // `exit` hook.
        let _gl_display = self.gl_context.take().unwrap().display();

        // Clear the window.
        self.state = None;
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(AppState { gl_surface, window }) = self.state.as_ref() {
            let gl_context = self.gl_context.as_ref().unwrap();
            let renderer = self.renderer.as_mut().unwrap();

            let now = Instant::now();
            let dt = now.duration_since(self.last_updated).as_secs_f32();
            self.last_updated = Instant::now();

            self.scene.update(dt);

            self.scene.render(renderer);

            window.request_redraw();

            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
}

struct AppState {
    gl_surface: Surface<WindowSurface>,
    window: Window,
}

enum GlDisplayState {
    Build(DisplayBuilder),
    Init,
}
