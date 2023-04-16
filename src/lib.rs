use std::{
    io::{Read, Result},
    path::Path,
    time::Instant,
};

use imgui::Condition;
use nalgebra_glm as glm;

use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{uniform, Surface};

use camera::CameraState;
use shapes::vertex::Vertex;
use shapes::{cube, pyramid};

mod camera;
mod shapes;

const TITLE: &str = "Is a Window";

glium::implement_vertex!(Vertex, position, tex_cords);

pub fn run() {
    let (event_loop, display) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&display);

    let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui_context, &display)
        .expect("failed to initialize renderer");

    let pyramid_vertices = pyramid::VERTICES;
    let indices = glium::index::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &pyramid::INDICES,
    )
    .unwrap();

    let cube = cube::VERTICES;
    let cube_indices = glium::index::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &cube::INDICES,
    )
    .unwrap();

    let vertex_shader_src = get_shaders("shaders/vertex.vert").unwrap();

    let fragment_shader_src = get_shaders("shaders/fragment.vert").unwrap();

    let light_fragment = get_shaders("shaders/lightfrag.vert").unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &pyramid_vertices).unwrap();
    let cube_buffer = glium::VertexBuffer::new(&display, &cube).unwrap();

    let pyramid_scale = glm::scaling(&glm::vec3(0.5, 0.5, 0.5));
    let pyramid_model = glm::Mat4::identity() * pyramid_scale;

    //Setting Up Light Source

    let mut x_light_position = -2.0;
    let mut y_light_position = 0.0;
    let mut z_light_position = 0.0;
    let mut scale_factor = 0.25;

    let cube_model = glm::Mat4::identity();

    //log::info!("{:#?}", pyramid_model);

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();
    let light_program =
        glium::Program::from_source(&display, &vertex_shader_src, &light_fragment, None).unwrap();
    let mut camera = CameraState::new();
    camera.set_position((0.0, 0.0, -1.5));
    camera.set_direction((0.0, 0.0, 1.0));

    let mut last_frame = Instant::now();
    let mut last_updated = Instant::now();

    let pyramid_model: [[f32; 4]; 4] = pyramid_model.into();
    let light_color: [f32; 3] = [1.0, 1.0, 1.0];

    // Standard winit event loop
    // We use the ImGui events instead of normal Winit events.
    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = std::time::Instant::now();
            imgui_context.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }
        Event::MainEventsCleared => {
            let gl_window = display.gl_window();
            winit_platform
                .prepare_frame(imgui_context.io_mut(), gl_window.window())
                .expect("Failed to prepare frame");
            gl_window.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
            std::thread::sleep(std::time::Duration::from_millis(2));
            let params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            let delta_time = last_updated.elapsed().as_secs_f32();
            camera.update(delta_time);
            // Create frame for the all important `&imgui::Ui`
            //
            let ui = imgui_context.frame();

            // Draw our example content
            ui.window("Light Source")
                .size([300.0, 150.0], Condition::FirstUseEver)
                .build(|| {
                    let scale_slider = ui.slider_config("Scale", 0.0, 10.0);
                    scale_slider.display_format("%.2f").build(&mut scale_factor);

                    let slider_x = ui.slider_config("x_Position", -5.0, 5.0);
                    slider_x.display_format("%.2f").build(&mut x_light_position);

                    let slider_y = ui.slider_config("y_Position", -5.0, 5.0);
                    slider_y.display_format("%.2f").build(&mut y_light_position);

                    let slider_z = ui.slider_config("z_Position", -5.0, 5.0);
                    slider_z.display_format("%.2f").build(&mut z_light_position);
                });

            // Setup for drawing
            let gl_window = display.gl_window();
            let mut frame = display.draw();

            // Renderer doesn't automatically clear window
            frame.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

            let light_pos: [f32; 3] = [x_light_position, y_light_position, z_light_position];

            let light_scale = glm::scaling(&glm::vec3(scale_factor, scale_factor, scale_factor));
            let transpose = glm::translation(&light_pos.into());
            let cube_model: [[f32; 4]; 4] = (cube_model * light_scale * transpose).into();

            let pyramid_uniforms = uniform! {
                lightColor: light_color,
                lightPos: light_pos,
                view: camera.get_view(),
                perspective: camera.get_perspective(),
                model: pyramid_model,
            };

            let cube_uniforms = uniform! {
            lightColor: light_color,
            lightPos: light_pos,
            view: camera.get_view(),
            perspective: camera.get_perspective(),
            model: cube_model,
                   };
            frame
                .draw(
                    &vertex_buffer,
                    &indices,
                    &program,
                    &pyramid_uniforms,
                    &params,
                )
                .unwrap();

            frame
                .draw(
                    &cube_buffer,
                    &cube_indices,
                    &light_program,
                    &cube_uniforms,
                    &params,
                )
                .unwrap();

            // Perform rendering
            winit_platform.prepare_render(ui, gl_window.window());
            let draw_data = imgui_context.render();
            renderer
                .render(&mut frame, draw_data)
                .expect("Rendering failed");

            frame.finish().expect("Failed to swap buffers");

            last_updated = Instant::now();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        event => {
            let gl_window = display.gl_window();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => camera.process_input(&input),
                any => {
                    winit_platform.handle_event(imgui_context.io_mut(), gl_window.window(), &any)
                }
            }
        }
    });
}

fn create_window() -> (EventLoop<()>, glium::Display) {
    let event_loop = EventLoop::new();
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(24);
    let builder = glium::glutin::window::WindowBuilder::new()
        .with_title(TITLE.to_owned())
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1920f64, 1080f64));
    let display =
        glium::Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    (event_loop, display)
}

fn imgui_init(display: &glium::Display) -> (imgui_winit_support::WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);

    let gl_window = display.gl_window();
    let window = gl_window.window();

    let dpi_mode = imgui_winit_support::HiDpiMode::Default;

    winit_platform.attach_window(imgui_context.io_mut(), window, dpi_mode);

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    (winit_platform, imgui_context)
}

//Returns the own customized shaders file. As long as path is valid
//
pub fn get_shaders<P>(shader_path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut shader_file = std::fs::File::open(shader_path)?;

    let mut shader = String::new();
    shader_file.read_to_string(&mut shader)?;

    Ok(shader)
}
