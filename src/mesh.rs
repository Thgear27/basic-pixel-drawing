use crate::vertex::Vertex3;
pub struct Mesh {
    pub vertices: Vec<Vertex3<f32>>,
    pub indices: Vec<Vertex3<i32>>,
}
