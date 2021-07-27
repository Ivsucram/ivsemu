pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct DisplayBuffer {
    pub db: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    pub scale: u32,
}

impl DisplayBuffer {
    pub fn new(scale: u32) -> DisplayBuffer {
        DisplayBuffer {
            db: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
            scale: scale,
        }
    }

    pub fn clear(&mut self) {
        self.db = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
    }
}
