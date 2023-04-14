use std::{
    io::{Read, Result},
    path::Path,
    time::Instant,
};

use camera::CameraState;
use glium::{glutin, uniform, Surface};
use shapes::pyramid;
use shapes::vertex::Vertex;

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

    let vertex_shader_src = get_shaders("shaders/vertex.vert").unwrap();

    let fragment_shader_src = get_shaders("shaders/fragment.vert").unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &pyramid_vertices).unwrap();

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();
    let mut camera = CameraState::new();
    camera.set_position((0.0, 0.0, -1.5));
    camera.set_direction((0.0, 0.0, 1.0));

    let mut last_updated = Instant::now();

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

        let uniforms = uniform! {
            view: camera.get_view(),
            perspective: camera.get_perspective(),
           scale_matrix: [
               [0.5, 0.0, 0.0, 0.0],
               [0.0, 0.5 ,0.0, 0.0],
               [0.0, 0.0, 0.5, 0.0],
               [0.0, 0.0, 0.0, 1.0f32]
           ],
           translation_matrix: [
               [1.0, 0.0, 0.0, 0.0],
               [0.0, 1.0 ,0.0, 0.0],
               [0.0, 0.0, 1.0, 0.0],
               [0.0, 0.0, 0.0, 1.0f32]
           ],
        };

        frame
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();

        //log::info!("{}", delta_time);
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
