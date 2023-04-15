#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_cords: [f32; 3],
}

#[derive(Clone, Copy)]
pub struct NormalVertex {
    pub position: [f32; 3],
    pub normals: [f32; 3],
}
