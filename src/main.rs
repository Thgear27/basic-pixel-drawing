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

fn main() {
    let mut engine = Engine::new();
    engine.run();
}
