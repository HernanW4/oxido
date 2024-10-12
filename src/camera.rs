use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

const SENSITIVITY: f32 = 0.1;
const ASPECT_RATIO: f32 = 1.0;
const MOVEMENT_SPEED: f32 = 1.0;

#[derive(Clone, Copy, Default, Debug)]
pub struct Camera {
    position: glm::Vec3,
    direction: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    yaw: f32,
    pitch: f32,

    last_cursor_pos: Option<(f64, f64)>,
    camera_settings: CameraSettings,

    moving_forward: bool,
    moving_backwards: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
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
            camera_settings: CameraSettings::new(SENSITIVITY),
            ..Default::default()
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.direction), &self.up)
    }

    pub fn get_projection_mat(&self) -> glm::Mat4 {
        glm::perspective(
            self.camera_settings.aspect_ratio,
            self.camera_settings.fovy,
            self.camera_settings.near,
            self.camera_settings.far,
        )
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

    pub fn update(&mut self, delta_time: f32) {
        let velocity = self.camera_settings.movement_speed * delta_time;

        if self.moving_forward {
            self.move_forward(velocity);
        }
        if self.moving_backwards {
            self.move_backward(velocity);
        }
        if self.moving_left {
            self.move_left(velocity);
        }
        if self.moving_right {
            self.move_right(velocity);
        }
        if self.moving_up {
            self.move_up(velocity);
        }
        if self.moving_down {
            self.move_down(velocity);
        }
    }
    // Add methods for camera movement
    fn move_forward(&mut self, distance: f32) {
        self.position += self.direction * distance;
    }

    fn move_backward(&mut self, distance: f32) {
        self.position -= self.direction * distance;
    }

    fn move_right(&mut self, distance: f32) {
        self.position += self.right * distance;
    }

    fn move_left(&mut self, distance: f32) {
        self.position -= self.right * distance;
    }
    fn move_up(&mut self, distance: f32) {
        self.position += self.up * distance;
    }
    fn move_down(&mut self, distance: f32) {
        self.position -= self.up * distance;
    }

    pub fn rotate(&mut self, yaw_offset: f32, pitch_offset: f32) {
        self.yaw += yaw_offset;
        self.pitch += pitch_offset;

        //log::debug!("{}, {}", self.yaw, self.pitch);

        // Constrain the pitch
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.update_camera_vectors();
    }

    pub fn process_input(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let is_pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => self.moving_forward = is_pressed,
                    PhysicalKey::Code(KeyCode::KeyS) => self.moving_backwards = is_pressed,
                    PhysicalKey::Code(KeyCode::KeyA) => self.moving_left = is_pressed,
                    PhysicalKey::Code(KeyCode::KeyD) => self.moving_right = is_pressed,
                    PhysicalKey::Code(KeyCode::Space) => self.moving_up = is_pressed,
                    PhysicalKey::Code(KeyCode::ControlLeft) => self.moving_down = is_pressed,
                    _ => {}
                }
            }
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

#[derive(Clone, Copy, Default, Debug)]
pub struct CameraSettings {
    movement_speed: f32,
    aspect_ratio: f32,
    sensitivity: f32,
    fovy: f32,
    near: f32,
    far: f32,
}

impl CameraSettings {
    pub fn new(sensitivity: f32) -> Self {
        CameraSettings {
            aspect_ratio: ASPECT_RATIO,
            sensitivity,
            movement_speed: MOVEMENT_SPEED,
            fovy: 45.0f32.to_radians(),
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn change_sensitivity(&mut self, new_sen: f32) {
        self.sensitivity = new_sen;
    }
}
