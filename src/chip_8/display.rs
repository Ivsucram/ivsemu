const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    scale: u32,
    off_color: sdl2::pixels::Color,
    on_color: sdl2::pixels::Color
}

impl Display {
    pub fn init(sdl_context: &sdl2::Sdl, scale: u32) -> Display {
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window("Chip-8", WIDTH as u32 * scale,  
                                                      HEIGHT as u32 * scale)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display {
            canvas: canvas,
            scale: scale,
            off_color: sdl2::pixels::Color::RGB(0, 0, 0),
            on_color: sdl2::pixels::Color::RGB(255, 255, 255)
        }
    }

    pub fn draw(self: &mut Display, buffer: &[[bool; HEIGHT]; WIDTH]) {
        self.canvas.set_draw_color(self.off_color);
        self.canvas.clear();

        self.canvas.set_draw_color(self.on_color);
        for (x, col) in buffer.iter().enumerate() {
            for (y, pixel) in col.iter().enumerate() {
                if *pixel {
                    let x = (x as u32 * self.scale) as i32;
                    let y = (y as u32 * self.scale) as i32;
                    let width = self.scale;
                    let height = self.scale;
                    self.canvas.fill_rect(sdl2::rect::Rect::new(x, y, width, height))
                        .expect("Failed to draw pixel");
                }
            }
        }

        self.canvas.present();
    }
}