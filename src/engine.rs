use crate::pixel_renderer;
use crate::window;

use pixel_renderer::PixelRenderer;
use raylib_ffi::colors::*;
use raylib_ffi::GetTime;
use window::Window;

const WINDOW_WIDTH: usize = 800; // Window width
const WINDOW_HEIGHT: usize = 600; // Window height

const CONTEXT_WIDTH: usize = WINDOW_WIDTH / 2; // Context width
const CONTEXT_HEIGHT: usize = WINDOW_HEIGHT / 2; // Context  height

pub struct Engine {
    window: Window,
    renderer: PixelRenderer,
}

impl Engine {
    pub fn new() -> Self {
        let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Renderer");
        window.init();

        let renderer =
            PixelRenderer::new(CONTEXT_WIDTH, CONTEXT_HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT);

        Self { window, renderer }
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

        let time = get_time();

        let ypos = time.sin() * 100.0;
        let xpos = time.cos() * 100.0;

        self.renderer
            .line((0.0, 0.0), (150.0 + xpos, 150.0 + ypos), WHITE);
        self.renderer.show_pixels();

        self.renderer.end_drawing();
    }
}

// TODO: Move this to a separate module
fn get_time() -> f64 {
    unsafe { GetTime() }
}
