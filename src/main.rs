#![allow(dead_code)]

mod engine;
mod pixel_renderer;
mod window;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.run();
}
