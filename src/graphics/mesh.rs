use super::vertex::Vertex;

#[derive(Default, Debug)]
pub struct MeshData {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl MeshData {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<Vertex> {
        self.vertices.as_mut()
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

pub struct RenderableMesh {
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,

    index_count: u32,
}

impl RenderableMesh {
    pub fn new(
        vao: glow::VertexArray,
        vbo: glow::Buffer,
        ebo: glow::Buffer,
        index_count: u32,
    ) -> Self {
        Self {
            vao,
            vbo,
            ebo,
            index_count,
        }
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    pub fn mesh_arrays_buffers(&self) -> (&glow::VertexArray, &glow::Buffer, &glow::Buffer) {
        (&self.vao, &self.vbo, &self.ebo)
    }
}
