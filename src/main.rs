#![allow(dead_code)]

mod engine;
mod mesh;
mod model;
mod pixel_renderer;
mod vertex;
mod window;

use engine::Engine;
use model::Model;

fn main() {
    let model = Model::load_from_file("assets/cube.obj");
    println!("Model: {:?}", model);

    let mut engine = Engine::new();
    engine.run();
}
