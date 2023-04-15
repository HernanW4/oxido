use std::{
    io::{Read, Result},
    path::Path,
    time::Instant,
};

use glm::translation;
use nalgebra_glm as glm;

use camera::CameraState;
use glium::{glutin, uniform, Surface};
use shapes::vertex::Vertex;
use shapes::{cube, pyramid};

mod camera;
mod shapes;

glium::implement_vertex!(Vertex, position, tex_cords);

pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

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

    let vertex_buffer = glium::VertexBuffer::new(&display, &pyramid_vertices).unwrap();
    let cube_buffer = glium::VertexBuffer::new(&display, &cube).unwrap();

    let pyramid_scale = glm::scaling(&glm::vec3(0.5, 0.5, 0.5));
    let pyramid_model = glm::Mat4::identity() * pyramid_scale;

    let cube_scale = glm::scaling(&glm::vec3(0.2, 0.2, 0.2));
    let cube_tranlate = glm::translation(&glm::vec3(0.5, 2.0, -2.0));
    let cube_model = glm::Mat4::identity() * cube_scale * cube_tranlate;

    log::info!("{:#?}", pyramid_model);

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();
    let mut camera = CameraState::new();
    camera.set_position((0.0, 0.0, -1.5));
    camera.set_direction((0.0, 0.0, 1.0));

    let mut last_updated = Instant::now();

    let pyramid_model: [[f32; 4]; 4] = pyramid_model.into();
    let cube_model: [[f32; 4]; 4] = cube_model.into();

    event_loop.run(move |event, _, control_flow| {
        std::thread::sleep(std::time::Duration::from_millis(2));
        let delta_time = last_updated.elapsed().as_secs_f32();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                ev => camera.process_input(&ev),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let pyramid_uniforms = uniform! {
        view: camera.get_view(),
        perspective: camera.get_perspective(),
        model: pyramid_model,
               };

        let cube_uniforms = uniform! {
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
                &program,
                &cube_uniforms,
                &params,
            )
            .unwrap();
        //log::info!("Camera At: {:?}", camera.get_position());
        //log::info!("Camera Looking At: {:?}", camera.get_direction());

        camera.update(delta_time);

        frame.finish().unwrap();
        last_updated = Instant::now();
    });
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
