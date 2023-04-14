use std::{
    io::{Read, Result},
    path::Path,
};

use glium::{glutin, uniform, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_cords: [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_cords);

pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let top_point = Vertex {
        position: [0.0, 0.5],
        tex_cords: [0.0, 1.0],
    };
    let bottom_left = Vertex {
        position: [-0.5, -0.5],
        tex_cords: [0.0, 0.0],
    };
    let bottom_right = Vertex {
        position: [0.5, -0.5],
        tex_cords: [1.0, 0.0],
    };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = get_shaders("shaders/vertex.vert").unwrap();

    let fragment_shader_src = get_shaders("shaders/fragment.vert").unwrap();

    let vertex_buffer =
        glium::VertexBuffer::new(&display, &vec![top_point, bottom_left, bottom_right]).unwrap();

    let program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
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

        frame.finish().unwrap();
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
