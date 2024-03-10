#![allow(dead_code)]
extern crate lazy_static;

mod engine;
mod matrix;
mod mesh;
mod model;
mod pixel_renderer;
mod vertex;
mod window;

use engine::Engine;
use matrix::Matrix;
use model::Model;
use vertex::Vertex3;

fn main() {
    let model = Model::load_from_file("assets/cube.obj");
    println!("Model: {:?}", model);

    let mut engine = Engine::new();
    engine.run();

    // Viewport matrix test
    let m = Matrix::viewport(400, 400);

    // Translation matrix test
    let mut m2 = Matrix::identity(4);

    m2.data[0][3] = 1.0;

    // Homogeneous coordinates test
    let h_coords_test = Vertex3 {
        x: 0.0,
        y: 0.5,
        z: 1.0,
    };

    println!("{:?}", m2 * m * h_coords_test);
}
