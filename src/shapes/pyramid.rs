use crate::shapes::vertex::Vertex;

pub const VERTICES: [Vertex; 6] = [
    // Front Face (z = +)
    Vertex {
        position: [0.0, 0.5, 0.0], //Top Point 0
        //
        tex_cords: [0.0, 0.0, 0.0],
    },
    //Front Face
    Vertex {
        position: [0.5, -0.5, 0.5], //Front Bottom right 1
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5], // Front Bottom Left 2
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, -0.5], //Back bottom right 3
        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5], //Back bottom left 4

        tex_cords: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.5, 0.0], //Dummy points

        tex_cords: [0.0, 0.0, 0.0],
    }, //
];

pub const INDICES: [u16; 18] = [
    2, 0, 1, //Front Face
    3, 0, 4, //Back Face
    1, 0, 3, // Right Face
    4, 0, 2, // Left Face
    3, 4, 2, // Base Face
    2, 1, 3,
];
