use std::os::raw::c_void;

use raylib_ffi::colors::*;
use raylib_ffi::*;

pub struct PixelRenderer {
    screen_buffer_data: Vec<Color>,
    screen_buffer: Image,
    screen_texture: Texture2D,
    context_width: usize,
    context_height: usize,
    window_width: usize,
    window_height: usize,
}

impl PixelRenderer {
    pub fn new(
        context_width: usize,
        context_height: usize,
        window_width: usize,
        window_height: usize,
    ) -> Self {
        let mut screen_buffer_data = vec![
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            };
            context_width * context_height
        ];

        let screen_buffer = Image {
            data: screen_buffer_data.as_mut_ptr() as *mut c_void,
            width: context_width as i32,
            height: context_height as i32,
            format: enums::PixelFormat::R8g8b8a8 as i32,
            mipmaps: 1,
        };

        let screen_texture = load_texture_from_image(screen_buffer);

        PixelRenderer {
            screen_buffer_data,
            screen_buffer,
            screen_texture,
            context_width,
            context_height,
            window_width,
            window_height,
        }
    }

    pub fn put_pixel(&mut self, x: f32, y: f32, color: Color) {
        self.screen_buffer_data[(y * self.context_height as f32 + x) as usize] = color;
    }

    pub fn show_pixels(&mut self) {
        update_texture(
            self.screen_texture,
            self.screen_buffer.data as *const c_void,
        );

        draw_texture(self.screen_texture, self.window_width, self.window_height);
    }

    pub fn clear_buffer(&mut self, color: Color) {
        for pixel in self.screen_buffer_data.iter_mut() {
            *pixel = color;
        }
    }

    pub fn begin_drawing(&self, color: Color) {
        begin_drawing(color);
    }

    pub fn end_drawing(&self) {
        end_drawing();
    }

    pub fn get_ctx_width(&self) -> usize {
        self.context_width
    }

    // pub fn get_ctx_height(&self) -> usize {
    //     self.context_height
    // }
}

// TODO: move these functions to another module
// TODO: Fix double copy of the buffer texture
fn draw_texture(screen_texture: Texture2D, window_width: usize, window_height: usize) {
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
                width: window_width as f32,
                height: window_height as f32,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            WHITE,
        );
    }
}

fn end_drawing() {
    unsafe {
        EndDrawing();
    }
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

fn load_texture_from_image(image: Image) -> Texture2D {
    unsafe { LoadTextureFromImage(image) }
}
