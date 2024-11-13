use crate::graphics::mesh::MeshData;

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Transform {
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn with_position(mut self, position: glm::Vec3) -> Self {
        self.position = position;
        self
    }
    pub fn with_rotation(mut self, rotation: glm::Vec3) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn with_scale(mut self, scale: glm::Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn translate(&mut self, offset: glm::Vec3) {
        self.position += offset;
    }

    pub fn transformation(&self) -> glm::Mat4 {
        let mut matrix = glm::Mat4::identity();

        matrix = glm::scale(&matrix, &self.scale);

        //Rotation
        matrix = glm::rotate(&matrix, self.rotation.z, &glm::vec3(0.0, 0.0, 1.0)); // z-axis for
        matrix = glm::rotate(&matrix, self.rotation.y, &glm::vec3(0.0, 1.0, 0.0)); // y-axis for
        matrix = glm::rotate(&matrix, self.rotation.x, &glm::vec3(1.0, 0.0, 0.0)); // x-axis for
                                                                                   //

        matrix = glm::translate(&matrix, &self.position);

        matrix
    }

    //Setters and Getters
    pub fn set_position(&mut self, position: glm::Vec3) {
        self.position = position;
    }
    pub fn set_rotation(&mut self, rotation: glm::Vec3) {
        self.rotation = rotation;
    }
    pub fn set_scale(&mut self, scale: glm::Vec3) {
        self.scale = scale;
    }

    pub fn position(&self) -> &glm::Vec3 {
        &self.position
    }
    pub fn rotation(&self) -> &glm::Vec3 {
        &self.rotation
    }
    pub fn scale(&self) -> &glm::Vec3 {
        &self.scale
    }
}

pub trait Component {}

#[derive(Default)]
#[allow(dead_code)]
pub struct Entity {
    transform: Transform,
    mesh_data: Option<MeshData>,
}

#[allow(dead_code)]
impl Entity {
    pub fn new() -> Self {
        Entity {
            transform: Transform::new(),
            mesh_data: None,
        }
    }

    pub fn with_mesh_data(mut self, mesh_data: MeshData) -> Self {
        self.mesh_data = Some(mesh_data);
        self
    }
    pub fn with_position(mut self, position: glm::Vec3) -> Self {
        self.transform = Transform::new().with_position(position);
        self
    }

    pub fn apply_transformation(
        &mut self,
        position: Option<glm::Vec3>,
        rotation: Option<glm::Vec3>,
        scale: Option<glm::Vec3>,
    ) {
        if let Some(new_position) = position {
            self.transform.set_position(new_position);
        }
        if let Some(new_rotation) = rotation {
            self.transform.set_rotation(new_rotation);
        }
        if let Some(new_scale) = scale {
            self.transform.set_scale(new_scale);
        }
    }

    pub fn transformation(&self) -> glm::Mat4 {
        self.transform.transformation()
    }

    pub fn mesh_data(&self) -> &Option<MeshData> {
        &self.mesh_data
    }
}
