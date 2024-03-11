use std::os::raw::c_void;

use raylib_ffi::colors::*;
use raylib_ffi::*;

pub struct Renderer {
    screen_buffer_data: Vec<Color>,
    screen_buffer: Image,
    screen_texture: Texture2D,
    context_width: usize,
    context_height: usize,
    window_width: usize,
    window_height: usize,
}

impl Renderer {
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

        Renderer {
            screen_buffer_data,
            screen_buffer,
            screen_texture,
            context_width,
            context_height,
            window_width,
            window_height,
        }
    }

    pub fn put_pixel(&mut self, x: f64, y: f64, color: Color) {
        // Assert that the pixel is within the context
        if x < 0.0 || x >= self.context_width as f64 || y < 0.0 || y >= self.context_height as f64 {
            println!("Pixel out of bounds!");
            return;
        }

        let y = self.context_height as f64 - y - 1.0;
        self.screen_buffer_data[(y * self.context_width as f64 + x) as usize] = color;
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

    pub fn triangle_line(&mut self, p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), color: Color) {
        self.line(p1, p2, color);
        self.line(p2, p3, color);
        self.line(p1, p3, color);
    }

    pub fn line(&mut self, mut start: (f32, f32), mut end: (f32, f32), color: Color) {
        // Slope should be less than 1
        let mut inverted_plane = false;
        if (end.1 - start.1).abs() > (end.0 - start.0).abs() {
            // Swap x and y
            std::mem::swap(&mut start.0, &mut start.1);
            std::mem::swap(&mut end.0, &mut end.1);
            inverted_plane = true;
        }

        // If the start in the right of the end, swap them
        // This step should only be done only after the past if
        if start.0 > end.0 {
            std::mem::swap(&mut start, &mut end);
        }

        // Using Bresenham's line algorithm
        let (x1, y1) = start;
        let (x2, y2) = end;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let from = x1.floor() as i32;
        let to = x2.floor() as i32;

        let mut dxe = dy - dx; // This is going to be incremented by dy in every iteration
        let mut y = y1.floor() as i32; // Initial value of y

        for x in from..to {
            // Draw the pixel
            if inverted_plane {
                self.put_pixel(y as f64, x as f64, color);
            } else {
                self.put_pixel(x as f64, y as f64, color);
            }

            dxe += dy;

            // Update the error
            if dxe >= 0.0 {
                if (y2 - y1) > 0.0 {
                    y += 1;
                } else {
                    y -= 1;
                }

                dxe -= dx;
            }
        }
    }

    pub fn get_ctx_height(&self) -> usize {
        self.context_height
    }
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
