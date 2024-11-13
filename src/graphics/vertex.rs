use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normals: glm::Vec3,
    pub colors: glm::Vec3,
}
