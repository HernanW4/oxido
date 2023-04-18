//pub mod math {
//
//    use nalgebra_glm as glm;
//
//    use crate::shapes::vertex::Vertex;
//    pub fn calculate_face_normals(vertices: &[[f32; 3]; 3], indices: &[u32]) -> Vec<[f32; 3]> {
//        indices
//            .windows(3)
//            .map(|indices| {
//                let v1 = vertices[indices[0] as usize];
//                let v2 = vertices[indices[1] as usize];
//                let v3 = vertices[indices[2] as usize];
//
//                let v1 = glm::Vec3::from([v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]]);
//                let v2 = glm::Vec3::from([v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]]);
//
//                glm::normalize(&glm::Vec3::cross(&v1, &v2)).into()
//            })
//            .collect::<Vec<[f32; 3]>>()
//    }
//}
