#![allow(dead_code)]

mod engine;
mod mesh;
mod pixel_renderer;
mod vertex;
mod window;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.run();
}
