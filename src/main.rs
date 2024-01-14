use std::ffi::c_void;

use raylib_ffi::*;
use raylib_ffi::colors::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    init_window(WIDTH, HEIGHT, "Renderer");

    let mut screen_buffer_data = [Color { r: 0, g: 0, b: 0, a: 255 }; WIDTH * HEIGHT];
    let screen_buffer = Image {
        data: screen_buffer_data.as_mut_ptr() as *mut c_void,
        width: (WIDTH / 2) as i32,
        height: (HEIGHT / 2) as i32,
        format: enums::PixelFormat::R8g8b8a8 as i32,
        mipmaps: 1,
    };

    let screen_texture = load_texture_from_image(screen_buffer);

    screen_buffer_data[150 * (WIDTH / 2) + 200] = RED;

    while !window_should_close() {
        unsafe {
            BeginDrawing();
            ClearBackground(BLACK);
            UpdateTexture(screen_texture, screen_buffer.data as *const c_void);
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
                    width: WIDTH as f32,
                    height: HEIGHT as f32,
                },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                WHITE
            );
            EndDrawing();
        }
    }

    close_window();
}

fn init_window(width: usize, height: usize, title: &str) {
    unsafe {
        InitWindow(width as i32, height as i32, (title.to_string() + "\0").as_ptr() as *const i8);
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
