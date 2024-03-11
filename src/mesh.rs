use crate::matrix::Matrix;
use crate::pixel_renderer::Renderer;
use crate::vertex::Vertex3;
use raylib_ffi::colors::WHITE;
use raylib_ffi::GetTime;
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

    pub fn draw(&self, renderer: &mut Renderer, viewport: &Matrix) {
        // Iterar todas la faces
        for face in &self.faces {
            let mut triangle: [Vertex3<f32>; 3] = [Vertex3::new(0.0, 0.0, 0.0); 3];
            for (index, vertex_index) in face.indices.iter().enumerate() {
                triangle[index] = self.vertices[*vertex_index as usize];
            }

            // Calculos con matrices
            for i in 0..3 {
                let viewport_matrix = viewport.clone(); // I HATE THIS LINE
                let time = get_time();
                let rotation_matrix = Matrix::rotation_y(time);
                let translation_matrix = Matrix::translation(0.0, -0.0, -5.0);
                let perspective_matrix = Matrix::projection(6.0 / 8.0, 0.5, 100.0, 700.0);
                let res_vertex = (viewport_matrix)
                    * perspective_matrix
                    * translation_matrix
                    * rotation_matrix
                    * (triangle[i]);
                triangle[i] = Vertex3::new(
                    res_vertex.data[0][0] / res_vertex.data[3][0],
                    res_vertex.data[1][0] / res_vertex.data[3][0],
                    res_vertex.data[2][0] / res_vertex.data[3][0],
                );
            }

            renderer.triangle_line(
                (triangle[0].x, triangle[0].y),
                (triangle[1].x, triangle[1].y),
                (triangle[2].x, triangle[2].y),
                WHITE,
            );
        }

        // Dibujar los puntos
    }
}

fn get_time() -> f32 {
    unsafe { GetTime() as f32 }
}
