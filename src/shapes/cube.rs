use crate::shapes::vertex::Vertex;

pub const VERTICES: [Vertex; 15] = [
    Vertex {
        position: [-0.5, -0.5, -0.5], //0: Bottom Left Back
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5], //1: Top Left Back
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, -0.5], //2: Top Right Back
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5], //3: Bottom Right Back
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5], //4: Bottom Left Front
        tex_cords: [0.0, 0.0, 0.0],
    },
    //
    //
    Vertex {
        position: [0.5, -0.5, 0.5], //5: Bottom Right Front
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5], //6: Top Left Front

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.5], //7: Top Right Front

        tex_cords: [0.0, 0.0, 0.0],
    },
    //
    //
    //Dummy Points
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],

        tex_cords: [0.0, 0.0, 0.0],
    },
];
pub const INDICES: [u16; 36] = [
    2, 1, 0, 3, 2, 0, // Front face
    7, 5, 4, 4, 6, 7, // Back face
    6, 4, 0, 0, 1, 6, // Left face
    3, 5, 7, 7, 2, 3, // Right face
    6, 1, 2, 2, 7, 6, // Top face
    3, 0, 4, 4, 5, 3, // Bottom face
];
