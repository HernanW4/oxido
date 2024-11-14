#[derive(Default, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Transform {
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
}

#[allow(dead_code)]
impl Transform {
    pub fn new() -> Self {
        Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(0.25, 0.25, 0.25),
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
