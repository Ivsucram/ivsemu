// CHIP-8 means Compact Hexadecimal Interpretive Programming - 8-bit
use ::sdl2;

use crate::chip_8::display;
use rand::Rng;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SCALE: u32 = 10;

pub struct ProgramCounter(usize);
pub struct Registers([u8; 16]);
pub struct Ram([u8; 4 * 1024]);
pub struct Timer(u8);
pub struct Timing(u16);
pub struct DisplayBuffer([[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH]);
pub struct Keypad {
    keys: [bool; 0xF],
    layout: String
}

impl ProgramCounter {
    fn init() -> ProgramCounter {
        ProgramCounter {
            0: 0
        }
    }

    fn increase(&mut self) {
        self.0 += 2;
    }

    fn set(&mut self, value: usize) {
        self.0 = value;
    }

    fn get(&self) -> usize {
        self.0
    }
}

impl Registers {
    fn init() -> Registers {
        Registers {
            0: [0x0; 16]
        }
    }

    fn get(&self, register: usize) -> u8 {
        if register <= 0xF {
            self.0[register]
        } else {
            0
        }
    }

    fn set(&mut self, register: usize, value: u8) {
        if register <= 0xF {
            self.0[register] = value;
        }
    }
}

impl Ram {
    fn init() -> Ram {
        Ram {
            0: [0x00; 4 * 1024]
        }
    }

    fn init_fonts(&mut self) {
        const FONTS: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0,  // 0
            0x20, 0x60, 0x20, 0x20, 0x70,  // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
            0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
            0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
            0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
            0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
            0xF0, 0x80, 0xF0, 0x80, 0x80]; // F
        for i in 0x50..0x80 {
            self.0[i] = FONTS[i];
        }
    }

    fn load_rom(&mut self, rom: &[u8] ) {
        assert!(rom.len() <= self.0.len() - 0x200);
        for i in 0..rom.len() {
            self.0[0x200 + i] = rom[i];
        }
    }

    fn read8(&self, addr: usize) -> u8 {
        assert!(addr <= self.0.len());
        self.0[addr]
    }

    fn read16(&self, addr: usize) -> u16 {
        assert!(addr < self.0.len());
        // byteorder::LittleEndiar::read_u16(&self.ram[addr])
        
        // u16::from_le_bytes(self.ram[addr..addr+2])
        // self.ram[addr..addr+2]
        (self.0[addr] as u16) << 8 | self.0[addr+1] as u16
    }
}

impl Timer {
    fn init() -> Timer {
        Timer {
            0: 0
        }
    }

    fn tick(&mut self) -> bool {
        self.0 -= 1;
        self.0 == 0
    }
}

impl Keypad {
    fn init() -> Keypad {
        Keypad {
            keys: [false; 0xF],
            layout: String::from("123C456D789EA0BF")
        }
    }

    fn set_layout(&mut self, layout: &str) {
        assert_eq!(layout.len(), self.layout.len());
        self.layout = layout.to_string();
    }
}

impl Timing {
    fn init() -> Timing {
        Timing {
            0: 1000 // 1 MHz
        }
    }

    fn increase(&mut self) {
        self.0 += 10;
    }

    fn decrease(&mut self) {
        self.0 -= 10;
    }

    fn format(&self) -> String {
        let mut res = String::new();
        if self.0 >= 1000 {
            res = format!("{:.2}{}", self.0/1000, "MHz");
        } else {
            res = format!("{}{}", self.0, "Hz");
        }
        res
    }
}

impl DisplayBuffer {
    fn init() -> DisplayBuffer {
        DisplayBuffer {
            0: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH]
        }
    }

    fn clear(&mut self) {
        self.0 = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
    }

    fn random(&mut self) {
        for (x, col) in self.0.iter_mut().enumerate() {
            for (y, pixel) in col.iter_mut().enumerate() {
                let mut rng = rand::thread_rng();
                *pixel = rng.gen_range(0..2) == 0;
            }
        }
    }
}

pub fn run() {
    let mut pc = ProgramCounter::init();
    let mut index_register: u16 = 0;
    let mut stack: Vec<u16>;
    let mut delay_timer = Timer::init();
    let mut sound_timer = Timer::init();
    let mut registers = Registers::init();
    let mut ram = Ram::init();
    let mut keypad = Keypad::init();
    let mut timing = Timing::init();
    let mut display_buffer = DisplayBuffer::init();

    let sdl_context = sdl2::init().unwrap();

    let mut display = crate::chip_8::display::Display::init(&sdl_context, DISPLAY_SCALE);

    let mut event_listener = sdl_context.event_pump().unwrap();
    let mut is_running = true;
    'runner: loop {
        for event in event_listener.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {break 'runner},
                _ => {}
            }
        }
        
        display_buffer.random();
        display.draw(&display_buffer.0)
    }
}

pub fn fetch(ram: Ram, pc: &mut ProgramCounter) -> u16{
    let opcode = ram.read16(pc.get());
    pc.increase();
    opcode
}

pub fn decode() {}

pub fn execute() {}


