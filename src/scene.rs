use winit::event::WindowEvent;

use crate::{camera::Camera, entity::Entity, renderer::Renderer};

pub struct Scene {
    entity: Vec<Entity>,
    camera: Camera,
}
impl Scene {
    pub fn new(camera_pos: glm::Vec3) -> Self {
        let camera = Camera::new(camera_pos, glm::vec3(0.0, 1.0, 0.0), -90.0, 0.0);

        Scene {
            entity: Vec::new(),
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

        for entity in &self.entity {
            renderer.render(entity);
        }
    }

    pub fn process_input(&mut self, event: WindowEvent) {
        match event {
            ev => self.camera.process_input(ev),
        }
    }
}
