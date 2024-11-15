use crate::graphics::vertex::Vertex;

pub enum Face {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Face {
    pub fn all() -> Vec<Face> {
        vec![
            Face::Front,
            Face::Back,
            Face::Bottom,
            Face::Top,
            Face::Left,
            Face::Right,
        ]
    }
    pub fn neighbor_direction(&self) -> glm::Vec3 {
        match self {
            Face::Top => glm::vec3(0.0, 1.0, 0.0),
            Face::Bottom => glm::vec3(0.0, -1.0, 0.0),
            Face::Front => glm::vec3(0.0, 0.0, -1.0),
            Face::Back => glm::vec3(0.0, 0.0, 1.0),
            Face::Left => glm::vec3(-1.0, 0.0, 0.0),
            Face::Right => glm::vec3(1.0, 0.0, 0.0),
        }
    }
    fn to_normal(&self) -> glm::Vec3 {
        match self {
            Face::Top => glm::vec3(0.0, 1.0, 0.0),
            Face::Bottom => glm::vec3(0.0, -1.0, 0.0),
            Face::Front => glm::vec3(0.0, 0.0, 1.0),
            Face::Back => glm::vec3(0.0, 0.0, -1.0),
            Face::Left => glm::vec3(-1.0, 0.0, 0.0),
            Face::Right => glm::vec3(1.0, 0.0, 0.0),
        }
    }
    pub fn vertices_of_face(&self, wp: glm::Vec3) -> Vec<Vertex> {
        let (x, y, z) = (wp.x, wp.y, wp.z);
        let normals = self.to_normal();
        match self {
            Face::Front => {
                vec![
                    // Front face vertices
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z - 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z - 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z - 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z - 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
            Face::Back => {
                vec![
                    //Back Face vertices
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z + 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z + 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z + 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z + 1.0),
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
            Face::Right => {
                vec![
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z - 1.0), // bottom front
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z + 1.0), // bottom back
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z + 1.0), // top back
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z - 1.0), // top front
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
            Face::Left => {
                // Left face (looking from negative X direction)
                vec![
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z + 1.0), // bottom back
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z - 1.0), // bottom front
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z - 1.0), // top front
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z + 1.0), // top back
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
            Face::Top => {
                // Top face (looking from positive Y direction)
                vec![
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z - 1.0), // front left
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z - 1.0), // front right
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y + 1.0, z + 1.0), // back right
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y + 1.0, z + 1.0), // back left
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
            Face::Bottom => {
                // Bottom face (looking from negative Y direction)
                vec![
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z + 1.0), // back left
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z + 1.0), // back right
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x + 1.0, y - 1.0, z - 1.0), // front right
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                    Vertex {
                        position: glm::vec3(x - 1.0, y - 1.0, z - 1.0), // front left
                        normals,
                        colors: glm::vec3(0.0, 1.0, 0.0),
                    },
                ]
            }
        }
    }


    #[rustfmt::skip]
    pub fn indices_of_face(vertices_offset: u32) -> [u32;6] {

        [
            vertices_offset + 0, vertices_offset + 1, vertices_offset + 2,     //
            vertices_offset + 2, vertices_offset + 3, vertices_offset + 0      //
        ]
    }
}
