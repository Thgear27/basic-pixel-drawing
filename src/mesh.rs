use crate::vertex::Vertex3;
use russimp::face::Face as RussFace;
use russimp::mesh::Mesh as RussMesh;

#[derive(Debug)]
pub struct Face {
    pub indices: Vec<u32>,
}

impl Face {
    pub fn new(russ_face: &RussFace) -> Face {
        let mut indices: Vec<u32> = vec![];

        for index in russ_face.0.iter() {
            indices.push(*index);
        }

        Face { indices }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex3<f32>>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn load_mesh(russ_mesh: &RussMesh) -> Mesh {
        let mut vertices: Vec<Vertex3<f32>> = vec![];
        let mut faces: Vec<Face> = vec![];

        for vertex in &russ_mesh.vertices {
            vertices.push(Vertex3 {
                x: vertex.x,
                y: vertex.y,
                z: vertex.z,
            });
        }

        for russ_face in &russ_mesh.faces {
            let face = Face::new(russ_face);
            faces.push(face);
        }

        Mesh { vertices, faces }
    }
}
