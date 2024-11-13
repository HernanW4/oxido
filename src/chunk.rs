use crate::graphics::vertex::Vertex;

const CHUNK_SIZE: usize = 16;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum BlockType {
    #[default]
    Air,
    Solid,
}

pub struct Chunk {
    blocks: Vec<BlockType>,
    position: glm::Vec3,
}

impl Chunk {
    pub fn new(position: glm::Vec3) -> Self {
        let size = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
        let blocks = Vec::with_capacity(size);

        Self { blocks, position }
    }

    pub fn get_position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn set_block(&mut self, pos: glm::Vec3, block_type: BlockType) {
        let index = Chunk::get_index_from_pos(pos);

        if let Some(block) = self.blocks.get_mut(index) {
            *block = block_type;
        }
    }
    pub fn get_block(&self, pos: glm::Vec3) -> Option<BlockType> {
        let index = Chunk::get_index_from_pos(pos);
        if let Some(block) = self.blocks.get(index) {
            return Some(*block);
        }
        None
    }
    fn get_index_from_pos(position: glm::Vec3) -> usize {
        position.x as usize
            + (position.y as usize * CHUNK_SIZE)
            + (position.z as usize * CHUNK_SIZE * CHUNK_SIZE)
    }

    pub fn generate_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let pos = glm::vec3(x as f32, y as f32, z as f32);
                    self.add_cube_to_mesh(pos, &mut vertices, &mut indices);
                }
            }
        }

        (vertices, indices)
    }

    fn add_cube_to_mesh(
        &self,
        position: glm::Vec3,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
    ) {
        let pos = position + self.position;

        let mut index_offset = vertices.len() as u32;

        // Define the 6 faces of a cube
        let faces = [
            (glm::vec3(0.0, 0.0, 1.0), glm::vec3(0.0, 1.0, 0.0)), // Front
            (glm::vec3(0.0, 0.0, -1.0), glm::vec3(0.0, 1.0, 0.0)), // Back
            (glm::vec3(1.0, 0.0, 0.0), glm::vec3(0.0, 1.0, 0.0)), // Right
            (glm::vec3(-1.0, 0.0, 0.0), glm::vec3(0.0, 1.0, 0.0)), // Left
            (glm::vec3(0.0, 1.0, 0.0), glm::vec3(0.0, 0.0, 1.0)), // Top
            (glm::vec3(0.0, -1.0, 0.0), glm::vec3(0.0, 0.0, -1.0)), // Bottom
        ];

        for (normal, up) in faces.iter() {
            if self.should_render(position, *normal) {
                let right = normal.cross(up);

                // Define the four corners of the face
                let corners = [
                    pos + *normal * 0.5 - right * 0.5 - *up * 0.5,
                    pos + *normal * 0.5 + right * 0.5 - *up * 0.5,
                    pos + *normal * 0.5 + right * 0.5 + *up * 0.5,
                    pos + *normal * 0.5 - right * 0.5 + *up * 0.5,
                ];

                for corner in corners.iter() {
                    vertices.push(Vertex {
                        position: *corner,
                        normals: *normal,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    });
                }

                indices.extend_from_slice(&[
                    index_offset,
                    index_offset + 1,
                    index_offset + 2,
                    index_offset,
                    index_offset + 2,
                    index_offset + 3,
                ]);

                // For every 4 corners we move the offset
                index_offset += 4;
            }
        }
    }

    fn should_render(&self, pos: glm::Vec3, normal: glm::Vec3) -> bool {
        let neighbor_pos = pos + normal;

        neighbor_pos.x as usize >= CHUNK_SIZE
            || neighbor_pos.y as usize >= CHUNK_SIZE
            || neighbor_pos.z as usize >= CHUNK_SIZE
            || self.get_block(neighbor_pos).unwrap_or_default() == BlockType::Air
    }
}
