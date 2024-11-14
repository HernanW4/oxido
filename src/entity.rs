use crate::{component::Transform, graphics::mesh::MeshData};

#[derive(Default, Debug)]
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

    pub fn cube_entity() -> Self {
        let data = create_cube_mesh_data();
        Entity {
            transform: Transform::new(),
            mesh_data: Some(data),
        }
    }

    pub fn set_position(&mut self, new_pos: glm::Vec3) {
        self.transform.set_position(new_pos);
    }

    pub fn position(&self) -> &glm::Vec3 {
        self.transform.position()
    }

    pub fn transformation(&self) -> glm::Mat4 {
        self.transform.transformation()
    }

    pub fn mesh_data(&self) -> &Option<MeshData> {
        &self.mesh_data
    }
    pub fn mesh_data_mut(&mut self) -> Option<&mut MeshData> {
        self.mesh_data.as_mut()
    }
}

fn create_cube_mesh_data() -> MeshData {
    use crate::graphics::vertex::Vertex;
    // Create a triangle mesh
    let cube_vertices = vec![
        -0.5, -0.5, 0.5, // Front face
        0.5, -0.5, 0.5, //
        0.5, 0.5, 0.5, //
        -0.5, 0.5, 0.5, //
        -0.5, -0.5, -0.5, // Back face
        0.5, -0.5, -0.5, //
        0.5, 0.5, -0.5, //
        -0.5, 0.5, -0.5, //
    ];
    let vertices: Vec<Vertex> = cube_vertices
        .chunks(3)
        .enumerate()
        .map(|(_i, v)| Vertex {
            position: glm::vec3(v[0], v[1], v[2]),
            normals: glm::Vec3::zeros(),
            colors: glm::vec3(0.0, 1.0, 0.0),
        })
        .collect();

    let indices = vec![
        // Front face
        0, 1, 2, //
        2, 3, 0, //
        // Back face
        4, 5, 6, //
        6, 7, 4, //
        // Left face
        4, 7, 3, //
        3, 0, 4, //
        // Right face
        1, 5, 6, //
        6, 2, 1, //
        // Top face
        3, 2, 6, //
        6, 7, 3, //
        // Bottom face
        0, 1, 5, //
        5, 4, 0, //
    ];

    MeshData::new(vertices, indices)
}
