use raylib_ffi::*;

pub struct Window {
    width: usize,
    height: usize,
    title: String,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
        }
    }

    pub fn init(&self) {
        unsafe {
            InitWindow(
                self.width as i32,
                self.height as i32,
                (self.title.to_string() + "\0").as_ptr() as *const i8,
            );
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn close_window(&self) {
        unsafe {
            CloseWindow();
        }
    }
}
