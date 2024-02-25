use std::ffi::c_void;

use raylib_ffi::colors::*;
use raylib_ffi::*;

const WINDOW_WIDTH: usize = 800; // Window width
const WINDOW_HEIGHT: usize = 600; // Window height

const CONTEXT_WIDTH: usize = WINDOW_WIDTH / 2; // Context width
const CONTEXT_HEIGHT: usize = WINDOW_HEIGHT / 2; // Context  height

struct Window {
    width: usize,
    height: usize,
    title: String,
}

impl Window {
    fn new(width: usize, height: usize, title: &str) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
        }
    }

    fn init(&self) {
        unsafe {
            InitWindow(
                self.width as i32,
                self.height as i32,
                (self.title.to_string() + "\0").as_ptr() as *const i8,
            );
        }
    }
}

struct Renderer {
    screen_buffer_data: [Color; CONTEXT_WIDTH * CONTEXT_HEIGHT],
    screen_buffer: Image,
    screen_texture: Texture2D,
}

impl Renderer {
    fn new() -> Self {
        let mut screen_buffer_data = [Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }; CONTEXT_WIDTH * CONTEXT_HEIGHT];

        let screen_buffer = Image {
            data: screen_buffer_data.as_mut_ptr() as *mut c_void,
            width: CONTEXT_WIDTH as i32,
            height: CONTEXT_HEIGHT as i32,
            format: enums::PixelFormat::R8g8b8a8 as i32,
            mipmaps: 1,
        };

        let screen_texture = load_texture_from_image(screen_buffer);

        Renderer {
            screen_buffer_data,
            screen_buffer,
            screen_texture,
        }
    }
}

fn main() {
    let window = Window::new(WINDOW_WIDTH, WINDOW_HEIGHT, "Renderer");
    window.init();

    let mut screen_buffer_data = [Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    }; (CONTEXT_WIDTH) * (CONTEXT_HEIGHT)];

    let screen_buffer = Image {
        data: screen_buffer_data.as_mut_ptr() as *mut c_void,
        width: (CONTEXT_WIDTH) as i32,
        height: (CONTEXT_HEIGHT) as i32,
        format: enums::PixelFormat::R8g8b8a8 as i32,
        mipmaps: 1,
    };

    let screen_texture = load_texture_from_image(screen_buffer);

    screen_buffer_data[(CONTEXT_HEIGHT / 2) * (CONTEXT_WIDTH) + (CONTEXT_WIDTH / 2)] = RED;

    while !window_should_close() {
        begin_drawing(BLACK);

        let time_in_seconds = get_time();

        let x = ((time_in_seconds).sin() * ((CONTEXT_WIDTH) as f64)).abs() as usize;

        println!("x: {}", x);

        screen_buffer_data[(CONTEXT_HEIGHT / 2) * (CONTEXT_WIDTH) + x] = WHITE;

        update_texture(screen_texture, screen_buffer.data as *const c_void);

        draw_texture(screen_texture);

        end_drawing();
        clear_buffer(BLACK, &mut screen_buffer_data);
    }

    close_window();
}

// TODO: Fix double copy of the buffer texture
fn draw_texture(screen_texture: Texture2D) {
    unsafe {
        DrawTexturePro(
            screen_texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: screen_texture.width as f32,
                height: screen_texture.height as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            WHITE,
        );
    }
}

fn clear_buffer(color: Color, buffer: &mut [Color]) {
    for pixel in buffer.iter_mut() {
        *pixel = color;
    }
}

fn end_drawing() {
    unsafe {
        EndDrawing();
    }
}

fn get_time() -> f64 {
    unsafe { GetTime() }
}

// TODO: Fix double copy of the buffer texture
fn update_texture(texture: Texture2D, data: *const ::std::os::raw::c_void) {
    unsafe {
        UpdateTexture(texture, data);
    }
}

fn begin_drawing(color: Color) {
    unsafe {
        BeginDrawing();
        ClearBackground(color);
    }
}

fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

fn close_window() {
    unsafe {
        CloseWindow();
    }
}

fn load_texture_from_image(image: Image) -> Texture2D {
    unsafe { LoadTextureFromImage(image) }
}
