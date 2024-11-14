use noise::{NoiseFn, OpenSimplex};

use crate::{entity::Entity, util::predetermined_etentities::create_cube};

#[derive(Debug)]
pub enum VoxelType {
    Air,
    Ground { color: glm::Vec3 },
}

impl VoxelType {
    pub fn is_visible(&self) -> bool {
        match self {
            VoxelType::Air => false,
            _ => true,
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
    body: Entity,
    voxel_type: VoxelType,
}

impl Voxel {
    pub fn new() -> Self {
        Voxel {
            body: create_cube(),
            voxel_type: VoxelType::Air,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.voxel_type.is_visible()
    }

    pub fn set_type(&mut self, voxel_type: VoxelType) {
        self.voxel_type = voxel_type;

        if let Some(mesh_data) = self.body.mesh_data_mut() {
            let color = self.voxel_type.get_color();

            for vertex in mesh_data.vertices_mut() {
                vertex.colors = color;
            }
        }
    }

    pub fn position(&self) -> &glm::Vec3 {
        self.body.position()
    }

    pub fn set_position(&mut self, new_pos: glm::Vec3) {
        self.body.set_position(new_pos);
    }

    pub fn entity(&self) -> &Entity {
        &self.body
    }
}

#[derive(Debug)]
pub struct Chunk {
    voxels: Vec<Voxel>,
}

pub const CHUNK_SIZE: u16 = 16;
pub const NUMBER_NEIGHBOR_TRESHOLD: u8 = 6;

#[allow(dead_code)]
impl Chunk {
    pub fn new(pos: glm::Vec3) -> Self {
        let vector_size = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

        let mut voxels: Vec<Voxel> = Vec::with_capacity(vector_size.into());

        Self::generate_chunks(pos, &mut voxels);

        Chunk { voxels }
    }

    fn generate_chunks(pos: glm::Vec3, list: &mut Vec<Voxel>) {
        let perlin = OpenSimplex::new(1);
        const NOISE_SCALE: f64 = 0.3;
        for z in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let mut voxel = Voxel::new();

                    let world_x = x as f64 + (pos.x * CHUNK_SIZE as f32) as f64;
                    let world_y = y as f64 + (pos.y * CHUNK_SIZE as f32) as f64;
                    let noise_height = perlin.get([world_x * NOISE_SCALE, world_y * NOISE_SCALE]);

                    let height = (noise_height + 1.0) * 16.0;

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
                    voxel.set_position(glm::vec3(
                        x as f32 + pos.x * CHUNK_SIZE as f32,
                        y as f32,
                        z as f32 + pos.z * CHUNK_SIZE as f32,
                    ));

                    list.push(voxel);
                }
            }
        }
    }

    pub fn entities(&self) -> Vec<&Entity> {
        self.voxels
            .iter()
            .filter(|voxel| voxel.is_visible())
            .filter(|voxel| self.is_visible_surface(*voxel.position()))
            .map(|voxel| voxel.entity())
            .collect()
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
                    .map_or(false, |voxel| voxel.is_visible());

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

    pub fn get_voxel(&mut self, pos: glm::Vec3) -> &Voxel {
        let index = self
            .index_from_pos(pos)
            .expect("Voxel does not exist at given pos: {pos}");

        self.voxels.get(index as usize).unwrap()
    }
    pub fn index_from_pos(&self, voxel_pos: glm::Vec3) -> Option<u64> {
        let (width, height) = (CHUNK_SIZE, CHUNK_SIZE);

        let (x, y, z) = (voxel_pos.x as u16, voxel_pos.y as u16, voxel_pos.z as u16);

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
