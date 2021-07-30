use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PITCH_BYTES: usize = std::mem::size_of::<u32>(); // 4 bytes: R G B A, from colors
const PITCH: usize = WIDTH * PITCH_BYTES;

pub struct Display {
    canvas: Canvas<Window>,
    off_color: [u8; PITCH_BYTES],
    on_color: [u8; PITCH_BYTES],
}

impl Display {
    pub fn init(sdl_context: &sdl2::Sdl, scale: u32) -> Display {
        let canvas = sdl_context.video().unwrap()
            .window("Chip-8", WIDTH as u32 * scale, HEIGHT as u32 * scale)
            .resizable().position_centered().build().unwrap()
            .into_canvas().build().unwrap();

        let off_color = Color::RGBA(0, 0, 0, 255);
        let on_color = Color::RGBA(255, 255, 255, 255);

        Display {
            canvas: canvas,
            off_color: [off_color.r, off_color.g, off_color.b, off_color.a],
            on_color: [on_color.r, on_color.g, on_color.b, on_color.a],
        }
    }

    pub fn draw(self: &mut Display, buffer: &[[bool; HEIGHT]; WIDTH]) {
        self.apply_texture(buffer);
        self.canvas.present();
    }

    fn apply_texture(&mut self, buffer: &[[bool; HEIGHT]; WIDTH]) {
        let mut framebuffer: [u8; HEIGHT * PITCH] = [0; HEIGHT * PITCH];
        for (x, col) in buffer.iter().enumerate() {
            for (y, pixel) in col.iter().enumerate() {
                let offset = x * PITCH_BYTES + y * PITCH;
                let color: &[u8; PITCH_BYTES] = if *pixel { &self.on_color } else { &self.off_color };
                for i in offset..(offset + PITCH_BYTES) { framebuffer[i] = color[i - offset]; }
            }
        }

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, WIDTH as u32, HEIGHT as u32)
            .unwrap();
        texture.update(None, &framebuffer, PITCH).unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
    }
}
