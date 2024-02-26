mod engine;
mod renderer;
mod window;

use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.run();
}
