use crate::renderer;
use crate::window;

use raylib_ffi::colors::*;
use raylib_ffi::GetTime;
use renderer::PixelRenderer;
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

        let time_in_seconds = get_time();

        let x = ((time_in_seconds).sin() * (self.renderer.get_ctx_width() as f64)).abs() as usize;

        println!("x: {}", x);

        self.renderer
            .put_pixel(x as f32, (self.renderer.get_ctx_width() / 2) as f32, WHITE);

        self.renderer.show_pixels();

        self.renderer.end_drawing();
        self.renderer.clear_buffer(BLACK);
        // Update game logic
    }
}

// TODO: Move this to a separate module
fn get_time() -> f64 {
    unsafe { GetTime() }
}
