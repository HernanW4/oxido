use winit::event::WindowEvent;

use crate::{
    camera::Camera,
    entity::Entity,
    renderer::Renderer,
    voxel::{Chunk, Voxel, VoxelType, CHUNK_SIZE},
};

pub struct Scene {
    entity: Vec<Entity>,
    chunks: Vec<Chunk>,
    camera: Camera,
}
impl Scene {
    pub fn new(camera_pos: glm::Vec3) -> Self {
        let camera = Camera::new(camera_pos, glm::vec3(0.0, 1.0, 0.0), -90.0, 0.0);

        Scene {
            entity: Vec::new(),
            chunks: Vec::new(),
            camera,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entity.push(entity);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.camera.update(delta_time);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.begin_frame(&self.camera);

        for chunk in &self.chunks {
            let mesh_data = chunk.generate_mesh();

            renderer.render_with_mesh(&mesh_data);
        }
    }

    pub fn set_scene(&mut self) {
        let chunks = 2;
        for z in -chunks..chunks {
            for x in -chunks..chunks {
                let position = glm::vec3(
                    x as f32 * CHUNK_SIZE as f32,
                    0.0,
                    z as f32 * CHUNK_SIZE as f32,
                );

                let chunk = Chunk::new(position);

                self.chunks.push(chunk);
            }
        }
    }

    pub fn process_input(&mut self, event: WindowEvent) {
        match event {
            ev => self.camera.process_input(ev),
        }
    }
}
