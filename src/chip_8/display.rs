use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const PITCH_BYTES: usize = std::mem::size_of::<u32>(); // 4 bytes: R G B A, from colors

pub struct Display {
    canvas: Canvas<Window>
}

impl Display {
    pub fn init(sdl_context: &sdl2::Sdl) -> Display {
        let scale = 10;
        let canvas = sdl_context.video().unwrap()
            .window("Chip-8", WIDTH as u32 * scale, HEIGHT as u32 * scale)
            .resizable().position_centered().build().unwrap()
            .into_canvas().build().unwrap();

        Display {
            canvas: canvas
        }
    }

    pub fn draw(self: &mut Display, frame_buffer: &[u8; HEIGHT * WIDTH * PITCH_BYTES]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, WIDTH as u32, HEIGHT as u32)
            .unwrap();
        texture.update(None, frame_buffer, WIDTH * PITCH_BYTES).unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();
    }
}
