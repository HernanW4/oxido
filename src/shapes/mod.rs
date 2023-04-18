use glium::{IndexBuffer, VertexBuffer};
use obj::load_obj;

// Loads the shaders from obj files
// This will be the preferrable method to display shapes
pub fn load_wavefront(
    display: &glium::Display,
    data: &[u8],
) -> (VertexBuffer<obj::Vertex>, IndexBuffer<u16>) {
    let obj = load_obj(data).unwrap();

    let vb = obj.vertex_buffer(display).unwrap();
    let ib = obj.index_buffer(display).unwrap();
    (vb, ib)
}

/// This is in case I want to make my own figures, like triangles :)
#[derive(Debug, Clone, Copy)]
pub struct MyVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 3],
}

glium::implement_vertex!(MyVertex, position, normal, texture);
