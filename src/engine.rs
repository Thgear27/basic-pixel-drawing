use crate::matrix::Matrix;
use crate::model::Model;
use crate::pixel_renderer;
use crate::window;

use pixel_renderer::Renderer;
use raylib_ffi::colors::*;
use raylib_ffi::GetTime;
use window::Window;

const WINDOW_WIDTH: usize = 800; // Window width
const WINDOW_HEIGHT: usize = 600; // Window height

const CONTEXT_WIDTH: usize = WINDOW_WIDTH / 2; // Context width
const CONTEXT_HEIGHT: usize = WINDOW_HEIGHT / 2; // Context  height

pub struct Engine {
    window: Window,
    renderer: Renderer,
    model: Model,
    viewport: Matrix,
}

impl Engine {
    pub fn new() -> Self {
        let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Renderer");
        window.init();

        let renderer = Renderer::new(CONTEXT_WIDTH, CONTEXT_HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT);

        let model = Model::load_from_file("assets/monkey.obj");

        let viewport = Matrix::viewport(CONTEXT_WIDTH, CONTEXT_HEIGHT);

        println!("Setup complete");
        Self {
            window,
            renderer,
            model,
            viewport,
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.update();
        }

        self.window.close_window();
    }

    fn update(&mut self) {
        self.renderer.begin_drawing(BLACK);
        self.renderer.clear_buffer(BLACK);

        self.model.draw(&mut self.renderer, &self.viewport);

        self.renderer.show_pixels();

        self.renderer.end_drawing();
    }
}

// TODO: Move this to a separate module
fn get_time() -> f32 {
    unsafe { GetTime() as f32 }
}
