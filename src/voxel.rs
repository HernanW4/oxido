use noise::{NoiseFn, OpenSimplex};

use crate::{face::Face, graphics::mesh::MeshData};

#[derive(Debug)]
pub enum VoxelType {
    Air,
    Ground { color: glm::Vec3 },
}

impl VoxelType {
    pub fn is_transparent(&self) -> bool {
        match self {
            VoxelType::Air => true,
            _ => false,
        }
    }

    pub fn get_color(&self) -> glm::Vec3 {
        match self {
            VoxelType::Ground { color } => *color,
            _ => glm::vec3(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct Voxel {
    voxel_type: VoxelType,
}

impl Voxel {
    pub fn new() -> Self {
        Voxel {
            voxel_type: VoxelType::Air,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.voxel_type.is_transparent()
    }

    pub fn set_type(&mut self, voxel_type: VoxelType) {
        self.voxel_type = voxel_type;
    }

    //pub fn position(&self) -> &glm::Vec3 {
    //    self.body.position()
    //}

    //pub fn set_position(&mut self, new_pos: glm::Vec3) {
    //    self.body.set_position(new_pos);
    //}

    //pub fn entity(&self) -> &Entity {
    //    &self.body
    //}
}

#[derive(Debug)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    position: glm::Vec3,
}

pub const CHUNK_SIZE: u16 = 16;
pub const NUMBER_NEIGHBOR_TRESHOLD: u8 = 6;

#[allow(dead_code)]
impl Chunk {
    pub fn new(pos: glm::Vec3) -> Self {
        let voxels = Self::generate_voxels_in_chunk(pos);

        Chunk {
            voxels,
            position: pos,
        }
    }

    fn generate_voxels_in_chunk(pos: glm::Vec3) -> Vec<Voxel> {
        let perlin = OpenSimplex::new(1);

        let flatten_size = CHUNK_SIZE.pow(3);
        let mut voxels = Vec::with_capacity(flatten_size.into());

        const NOISE_SCALE: f64 = 0.1;
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let mut voxel = Voxel::new();

                    let world_x = x as f64 + pos.x as f64;
                    let world_z = z as f64 + pos.z as f64;

                    let noise_height = perlin.get([world_x * NOISE_SCALE, world_z * NOISE_SCALE]);

                    let height = (noise_height + 1.0) * 4.0;

                    //log::debug!("Noise height: {noise_height:?}");
                    //log::debug!("height: {height:?}");

                    if y as f64 <= height {
                        let depth = (height - y as f64) / height;

                        let surface_color = glm::vec3(0.6, 0.3, 0.1);
                        let deep_color = glm::vec3(0.2, 0.1, 0.02);
                        let block_color = glm::vec3(
                            surface_color.x * (1.0 - depth as f32) + deep_color.x * depth as f32,
                            surface_color.y * (1.0 - depth as f32) + deep_color.y * depth as f32,
                            surface_color.z * (1.0 - depth as f32) + deep_color.z * depth as f32,
                        );

                        voxel.set_type(VoxelType::Ground { color: block_color });
                    }

                    voxels.push(voxel);
                }
            }
        }

        voxels
    }

    pub fn generate_mesh(&self) -> MeshData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let world_x = x as f32 + self.position.x;
                    let world_y = y as f32 + self.position.y;
                    let world_z = z as f32 + self.position.z;

                    let voxel = self
                        .get_voxel(glm::vec3(world_x, world_y, world_z))
                        .expect("Voxel should exists");

                    if voxel.is_transparent() {
                        continue;
                    }

                    let position = glm::vec3(world_x, world_y, world_z);

                    for neighbor_dir in Face::all() {
                        if self.should_render_face(position, &neighbor_dir) {
                            let face_vertices = neighbor_dir.vertices_of_face(position);
                            let vertex_offset = vertices.len() as u32;
                            let face_indices = Face::indices_of_face(vertex_offset);

                            vertices.extend_from_slice(face_vertices.as_slice());
                            indices.extend(face_indices.iter());
                        }
                    }
                }
            }
        }
        log::debug!("Added vertices!! {}", vertices.len());
        log::debug!("Added indices!! {}", indices.len());

        MeshData::new(vertices, indices)
    }

    //pub fn entities(&self) -> Vec<&Entity> {
    //    self.voxels
    //        .iter()
    //        .filter(|voxel| voxel.is_transparent())
    //        .filter(|voxel| self.is_visible_surface(*voxel.position()))
    //        .map(|voxel| MeshData::new())
    //        .collect()
    //}
    //

    pub fn should_render_face(&self, pos: glm::Vec3, face: &Face) -> bool {
        let current = self.get_voxel(pos);

        if current.is_none() {
            return true;
        }

        let current = current.unwrap();

        if current.is_transparent() {
            return false;
        }

        // Get coordinates of the neighboring voxel in the given direction

        let neighbor_direction = face.neighbor_direction();

        let neighbor_pos = pos + neighbor_direction;

        match self.get_voxel(neighbor_pos) {
            None => true,
            Some(voxel) => voxel.is_transparent(),
        }
    }

    pub fn is_visible_surface(&self, pos: glm::Vec3) -> bool {
        let neighbor_directions: [glm::Vec3; 6] = [
            glm::vec3(1.0, 0.0, 0.0),
            glm::vec3(-1.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0),
            glm::vec3(0.0, -1.0, 0.0),
            glm::vec3(0.0, 0.0, 1.0),
            glm::vec3(0.0, 0.0, -1.0),
        ];
        let mut number_of_neighbors = 0;
        for neighbor in neighbor_directions {
            let neighbor = pos + neighbor;
            if let Some(index) = self.index_from_pos(neighbor) {
                let neighbor_exists = self //Check if there is a neighbor
                    .get_with_index(index as usize)
                    .map_or(false, |voxel| voxel.is_transparent());

                //Sum it up if that is the case
                if neighbor_exists {
                    number_of_neighbors += 1;
                    if number_of_neighbors >= NUMBER_NEIGHBOR_TRESHOLD {
                        return false;
                    }
                }
            }
        }

        //log::debug!("Number of neighbors {number_of_neighbors}");
        //log::debug!("For Voxel at {pos:?}");

        true
    }

    fn get_with_index(&self, index: usize) -> Option<&Voxel> {
        self.voxels.get(index)
    }

    pub fn get_voxel(&self, pos: glm::Vec3) -> Option<&Voxel> {
        if let Some(index) = self.index_from_pos(pos) {
            return self.get_with_index(index as usize);
        }
        None
    }
    pub fn index_from_pos(&self, voxel_pos: glm::Vec3) -> Option<u64> {
        let local_pos = voxel_pos - self.position;
        let (width, height) = (CHUNK_SIZE, CHUNK_SIZE);

        let (x, y, z) = (
            local_pos.x.round() as u16,
            local_pos.y.round() as u16,
            local_pos.z.round() as u16,
        );

        //Check if the position is within chunk bounds
        if x >= CHUNK_SIZE || y >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return None;
        }

        let index = width * height * z + width * y + x;

        if self.index_exists(index) {
            return Some(index.into());
        }
        None
    }

    pub fn index_exists(&self, index: u16) -> bool {
        self.voxels.get(index as usize).is_some()
    }
}
