use sdl2::pixels::Color;
use crate::chip_8::display;

pub const WIDTH: usize = display::WIDTH;
pub const HEIGHT: usize = display::HEIGHT;
pub const PITCH_BYTES: usize = display::PITCH_BYTES;

pub struct FrameBuffer {
    pub frame_buffer: [u8; WIDTH * HEIGHT * PITCH_BYTES],
    pub toggle_buffer: [[bool; HEIGHT]; WIDTH],
    off_color: [u8; PITCH_BYTES],
    on_color: [u8; PITCH_BYTES],
}

impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        let off_color = Color::RGBA(0, 0, 0, 255);
        let on_color = Color::RGBA(255, 255, 255, 255);

        FrameBuffer {
            frame_buffer: [0; WIDTH * HEIGHT * PITCH_BYTES],
            toggle_buffer: [[false; HEIGHT]; WIDTH],
            off_color: [off_color.r, off_color.g, off_color.b, off_color.a],
            on_color: [on_color.r, on_color.g, on_color.b, on_color.a],
        }
    }

    pub fn clear(&mut self) {
        for (x, col) in self.toggle_buffer.iter().enumerate() {
            for (y, _) in col.iter().enumerate() {
                let offset = x * PITCH_BYTES + y * WIDTH * PITCH_BYTES;
                for i in offset..(offset + PITCH_BYTES) { self.frame_buffer[i] = self.off_color[i - offset]; }
            }
        }

        self.toggle_buffer = [[false; HEIGHT]; WIDTH];
    }

    pub fn update_frame_buffer(&mut self, x: usize, y: usize) {
        let offset = x * PITCH_BYTES + y * WIDTH * PITCH_BYTES;
        let color: &[u8; PITCH_BYTES] = if self.toggle_buffer[x][y] { &self.on_color } else { &self.off_color };
        for i in offset..(offset + PITCH_BYTES) { self.frame_buffer[i] = color[i - offset]; }
    }
}
