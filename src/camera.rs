use winit::{
    event::WindowEvent,
    keyboard::{KeyCode, PhysicalKey},
};

use log;

const SENSITIVITY: f32 = 0.1;

pub struct Camera {
    position: glm::Vec3,
    direction: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    yaw: f32,
    pitch: f32,

    last_cursor_pos: Option<(f64, f64)>,
}

impl Camera {
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera {
            position,
            direction: glm::vec3(0.0, 0.0, -1.0),
            up: glm::Vec3::zeros(),
            right: glm::Vec3::zeros(),
            world_up: up,
            yaw,
            pitch,
            last_cursor_pos: None,
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.direction), &self.up)
    }

    pub fn update_camera_vectors(&mut self) {
        let direction = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.direction = glm::normalize(&direction);
        self.right = glm::normalize(&glm::cross(&self.direction, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.direction));
    }

    // Add methods for camera movement
    pub fn move_forward(&mut self, distance: f32) {
        self.position += self.direction * distance;
    }

    pub fn move_backward(&mut self, distance: f32) {
        self.position -= self.direction * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        self.position += self.right * distance;
    }

    pub fn move_left(&mut self, distance: f32) {
        self.position -= self.right * distance;
    }

    pub fn rotate(&mut self, yaw_offset: f32, pitch_offset: f32) {
        self.yaw += yaw_offset;
        self.pitch += pitch_offset;

        // Constrain the pitch
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.update_camera_vectors();
    }

    pub fn process_input(&mut self, distance: f32, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => self.move_forward(distance),
                PhysicalKey::Code(KeyCode::KeyS) => self.move_backward(distance),
                PhysicalKey::Code(KeyCode::KeyA) => self.move_left(distance),
                PhysicalKey::Code(KeyCode::KeyD) => self.move_right(distance),
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((last_x, last_y)) = self.last_cursor_pos {
                    let (x, y): (f64, f64) = position.into();
                    let x_offset = (x - last_x) as f32 * SENSITIVITY;
                    let y_offset = (last_y - y) as f32 * SENSITIVITY;

                    self.rotate(x_offset, y_offset);
                }
                self.last_cursor_pos = Some(position.into());
            }
            _ => {}
        }
    }
}
