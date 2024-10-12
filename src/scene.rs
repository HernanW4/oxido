use glow::Context;
use winit::event::WindowEvent;

use crate::{
    camera::Camera,
    mesh::{Mesh, Vertex},
};

pub struct Object {
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,

    mesh: Mesh,
}

impl Object {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let vertices: Vec<Vertex> = vertices
            .chunks(3)
            .enumerate()
            .map(|(i, v)| Vertex {
                position: glm::vec3(v[0], v[1], v[2]),
                normals: glm::Vec3::zeros(),
                colors: glm::vec3(0.0, 1.0, 0.0),
            })
            .collect();
        log::debug!("Vertex {:?}", vertices);
        let mesh = Mesh::new(vertices, indices);

        Object {
            position: glm::Vec3::zeros(),
            rotation: glm::Vec3::zeros(),
            scale: glm::vec3(0.5, 0.5, 0.5),
            mesh,
        }
    }

    pub fn get_transformation(&self) -> glm::Mat4 {
        let translation = glm::translate(&glm::Mat4::identity(), &self.position);

        // 2. Create the rotation matrices
        let rotation_matrix_x = glm::rotate_x(&glm::Mat4::identity(), self.rotation.x);
        let rotation_matrix_y = glm::rotate_y(&glm::Mat4::identity(), self.rotation.y);
        let rotation_matrix_z = glm::rotate_z(&glm::Mat4::identity(), self.rotation.z);

        // Combine the rotations
        let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

        let scale_matrix = glm::scaling(&self.scale);

        translation * rotation_matrix * scale_matrix
    }

    pub fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn update(&mut self, delta_time: f32) {
        self.rotation += glm::vec3(delta_time * 1.0, 0.5 * delta_time, 2.0 * delta_time);
    }

    pub fn set_pos(&mut self, pos: glm::Vec3) {
        self.position = pos;
    }
}

pub struct Scene {
    objects: Vec<Object>,
    camera: Camera,
}

impl Scene {
    pub fn new(camera_pos: glm::Vec3) -> Self {
        let camera = Camera::new(camera_pos, glm::vec3(0.0, 1.0, 0.0), -90.0, 0.0);
        Scene {
            objects: Vec::new(),
            camera,
        }
    }

    pub fn add_objects(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_camera_attributes(&self) -> (glm::Mat4, glm::Mat4) {
        (
            self.camera.get_view_matrix(),
            self.camera.get_projection_mat(),
        )
    }

    pub fn get_objects(&mut self) -> &mut Vec<Object> {
        &mut self.objects
    }

    pub fn update(&mut self, delta_time: f32) {
        self.objects
            .iter_mut()
            .for_each(|obj| obj.update(delta_time));

        self.camera.update(delta_time);
    }

    pub fn process_input(&mut self, event: WindowEvent) {
        match event {
            ev => self.camera.process_input(ev),
        }
    }
}
