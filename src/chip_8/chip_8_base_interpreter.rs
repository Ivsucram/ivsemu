// CHIP-8 means Compact Hexadecimal Interpretive Programming - 8-bit
use ::sdl2;

use rand::Rng;

use std::io::Read;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SCALE: u32 = 10;

pub struct ProgramCounter(usize);
pub struct Registers{
    x_0: u8, x_1: u8, x_2: u8, x_3: u8, x_4: u8, x_5: u8, x_6: u8, x_7: u8,
    x_8: u8, x_9: u8, x_a: u8, x_b: u8, x_c: u8, x_d: u8, x_e: u8, x_f: u8
}
pub struct Ram([u8; 4 * 1024]);
pub struct DisplayBuffer([[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH]);

pub struct Timer{
    tick: u8,
    clock_hz: u128,
    elapsed: std::time::SystemTime
}

pub struct Keypad { 
    key_status: [bool; 0x10],
    keys: std::collections::HashMap<sdl2::keyboard::Keycode, usize>
}

pub struct OpCodes { 
    opcode: u16, 
    n1: u8, 
    n2: u8, 
    n3: u8, 
    n4: u8,
    x: usize,
    y: usize,
    n: u8,
    nn: u8,
    nnn: usize
}

impl ProgramCounter {
    fn init() -> ProgramCounter {
        ProgramCounter {
            0: 0x200
        }
    }

    fn increase(&mut self) {
        self.0 += 2;
    }

    fn decrement(&mut self) {
        self.0 -= 2;
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
            x_0: 0, x_1: 0, x_2: 0, x_3: 0, x_4: 0, x_5: 0, x_6: 0, x_7: 0,
            x_8: 0, x_9: 0, x_a: 0, x_b: 0, x_c: 0, x_d: 0, x_e: 0, x_f: 0
        }
    }

    fn get(&self, register: usize) -> u8 {
        assert!(register <= 0xF);
        match register {
            0x0 => self.x_0, 0x1 => self.x_1, 0x2 => self.x_2, 0x3 => self.x_3,
            0x4 => self.x_4, 0x5 => self.x_5, 0x6 => self.x_6, 0x7 => self.x_7,
            0x8 => self.x_8, 0x9 => self.x_9, 0xA => self.x_a, 0xB => self.x_b,
            0xC => self.x_c, 0xD => self.x_d, 0xE => self.x_e, 0xF => self.x_f,
            _ => 0
        }
    }

    fn set(&mut self, register: usize, value: u8) {
        assert!(register <= 0xF);
        match register {
            0x0 => self.x_0 = value, 0x1 => self.x_1 = value, 0x2 => self.x_2 = value, 0x3 => self.x_3 = value,
            0x4 => self.x_4 = value, 0x5 => self.x_5 = value, 0x6 => self.x_6 = value, 0x7 => self.x_7 = value,
            0x8 => self.x_8 = value, 0x9 => self.x_9 = value, 0xA => self.x_a = value, 0xB => self.x_b = value,
            0xC => self.x_c = value, 0xD => self.x_d = value, 0xE => self.x_e = value, 0xF => self.x_f = value,
            _ => {}
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
        let font_address = 0x50;
        let fonts: [u8; 80] = [
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
        for i in 0..fonts.len() {
            self.0[i + font_address] = fonts[i];
        }
    }

    fn load_rom(&mut self, rom: &[u8] ) {
        assert!(rom.len() <= self.0.len() - 0x200);
        for i in 0..rom.len() {
            self.0[0x200 + i] = rom[i];
        }
    }

    fn read8(&self, addr: usize) -> u8 {
        assert!(addr <= self.0.len(), "addr = {}, self.0.len() = {}", addr, self.0.len());
        self.0[addr]
    }

    fn read16(&self, addr: usize) -> u16 {
        assert!(addr < self.0.len(), "addr = {}, self.0.len() = {}", addr, self.0.len());
        // byteorder::LittleEndian::read_u16(&self.0[addr]);
        
        // u16::from_le_bytes(self.ram[addr..addr+2])
        // self.ram[addr..addr+2]
        (self.0[addr] as u16) << 8 | self.0[addr+1] as u16
    }

    fn write8(&mut self, addr: usize, value: u8) {
        assert!(addr < self.0.len());
        self.0[addr] = value;
    }
}

impl Timer {
    fn init() -> Timer {
        Timer {
            tick: 255,
            clock_hz: 60,
            elapsed: std::time::SystemTime::now()
        }
    }

    fn tick(&mut self) -> bool {
        let mut res: bool = false;
        match self.elapsed.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs_f32() >= 1./(self.clock_hz as f32)  {
                    if self.tick > 0 { self.tick -= 1; }
                    self.reset_elapsed();
                    res = true;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        res
    }

    fn reset_elapsed(&mut self) {
        self.elapsed = std::time::SystemTime::now();
    }
}

impl Keypad {
    fn init() -> Keypad {
        Keypad {
            key_status: [false; 0x10],
            keys: [(sdl2::keyboard::Keycode::Num1, 0x1),
                   (sdl2::keyboard::Keycode::Num2, 0x2),
                   (sdl2::keyboard::Keycode::Num3, 0x3),
                   (sdl2::keyboard::Keycode::Num4, 0xC),
                   (sdl2::keyboard::Keycode::Q, 0x4),
                   (sdl2::keyboard::Keycode::W, 0x5),
                   (sdl2::keyboard::Keycode::E, 0x6),
                   (sdl2::keyboard::Keycode::R, 0xD),
                   (sdl2::keyboard::Keycode::A, 0x7),
                   (sdl2::keyboard::Keycode::S, 0x8),
                   (sdl2::keyboard::Keycode::D, 0x9),
                   (sdl2::keyboard::Keycode::F, 0xE),
                   (sdl2::keyboard::Keycode::Z, 0xA),
                   (sdl2::keyboard::Keycode::X, 0x0),
                   (sdl2::keyboard::Keycode::C, 0xB),
                   (sdl2::keyboard::Keycode::V, 0xF)
                  ].iter().cloned().collect()
        }
    }

    fn compute_keycode(&self, keycode: sdl2::keyboard::Keycode) -> Option<usize> {
        match self.keys.get(&keycode) {
            Some(chip8_key) => Some(*chip8_key),
            None => None
        }
    }

    fn get(&mut self, pos: usize) -> bool {
        self.key_status[pos]
    }

    fn being_pressed(&self) -> Option<u8> {
        for key in 0x0..0x10 {
            if self.key_status[key] {
                return Some(key as u8)
            }
        }
        None
    }

    fn press(&mut self, key: usize) {
        self.key_status[key] = true;
    }

    fn release(&mut self, key: usize) {
        self.key_status[key] = false;
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
}

impl OpCodes {
    fn init(opcode: u16) -> OpCodes {
        let n1  = (opcode & 0xF000) >> 12;
        let n2  = (opcode & 0xF00) >> 8;
        let n3  = (opcode & 0xF0) >> 4;
        let n4  =  opcode & 0xF;
        let nn  =  opcode & 0xFF;
        let nnn =  opcode & 0xFFF;

        OpCodes {
            opcode: opcode,
            n1: n1 as u8,
            n2: n2 as u8,
            n3: n3 as u8,
            n4: n4 as u8,
            x: n2 as usize,
            y: n3 as usize,
            n: n4 as u8,
            nn: nn as u8,
            nnn: nnn as usize
        }
    }    
}

struct CPU {
    pc: ProgramCounter,
    index_register: usize,
    stack: Vec<usize>,
    delay_timer: Timer,
    sound_timer: Timer,
    clock: Timer,
    registers: Registers,
    ram: Ram,
    keypad: Keypad,
    display_buffer: DisplayBuffer,
    opcode: OpCodes,

    str_to_hex_map: std::collections::HashMap<char, Option<u8>>
}

impl CPU {
    fn init() -> CPU {
        CPU {
            pc: ProgramCounter::init(),
            index_register: 0,
            stack: vec![],
            delay_timer: Timer::init(),
            sound_timer: Timer::init(),
            clock: Timer::init(),
            registers: Registers::init(),
            ram: Ram::init(),
            keypad: Keypad::init(),
            display_buffer: DisplayBuffer::init(),
            opcode: OpCodes::init(0000),

            str_to_hex_map: [('0', Some(0x0)), ('1', Some(0x1)), ('2', Some(0x2)), ('3', Some(0x3)),
                             ('4', Some(0x4)), ('5', Some(0x5)), ('6', Some(0x6)), ('7', Some(0x7)),
                             ('8', Some(0x8)), ('9', Some(0x9)), ('A', Some(0xA)), ('B', Some(0xB)),
                             ('C', Some(0xC)), ('D', Some(0xD)), ('E', Some(0xE)), ('F', Some(0xF)),
                             ('?', None)].iter().cloned().collect()
        }
    }

    fn fetch(&mut self) {
        self.opcode = OpCodes::init(self.ram.read16(self.pc.get()));
        self.pc.increase();
    }

    fn decode(&mut self) {
        if self.decode_match("00E0") { op_00e0(self);
        } else if self.decode_match("1???") { op_1nnn(self);
        } else if self.decode_match("00EE") { op_00ee(self);
        } else if self.decode_match("2???") { op_2nnn(self);
        } else if self.decode_match("3???") { op_3xnn(self);
        } else if self.decode_match("4???") { op_4xnn(self);
        } else if self.decode_match("5??0") { op_5xy0(self);
        } else if self.decode_match("9??0") { op_9xy0(self);
        } else if self.decode_match("6???") { op_6xnn(self);
        } else if self.decode_match("7???") { op_7xnn(self);
        } else if self.decode_match("8??0") { op_8xy0(self);
        } else if self.decode_match("8??1") { op_8xy1(self);
        } else if self.decode_match("8??2") { op_8xy2(self);
        } else if self.decode_match("8??3") { op_8xy3(self);
        } else if self.decode_match("8??4") { op_8xy4(self);
        } else if self.decode_match("8??5") { op_8xy5(self);
        } else if self.decode_match("8??6") { op_8xy6(self);
        } else if self.decode_match("8??7") { op_8xy7(self);
        } else if self.decode_match("8??E") { op_8xye(self);
        } else if self.decode_match("A???") { op_annn(self);
        } else if self.decode_match("B???") { op_bnnn(self);
        } else if self.decode_match("C???") { op_cxnn(self);
        } else if self.decode_match("D???") { op_dxyn(self);
        } else if self.decode_match("E?9E") { op_ex9e(self);
        } else if self.decode_match("E?A1") { op_exa1(self);
        } else if self.decode_match("F??7") { op_fx07(self);
        } else if self.decode_match("F?15") { op_fx15(self);
        } else if self.decode_match("F?18") { op_fx18(self);
        } else if self.decode_match("F?1E") { op_fx1e(self);
        } else if self.decode_match("F?0A") { op_fx0a(self);
        } else if self.decode_match("F?29") { op_fx29(self);
        } else if self.decode_match("F?33") { op_fx33(self);
        } else if self.decode_match("F?55") { op_fx55(self);
        } else if self.decode_match("F?65") { op_fx65(self);
        } else {
            println!{"Unknown instruction: {:04x}", self.opcode.opcode};
        }
    }

    fn decode_match(&self, hex_code: &str) -> bool {
        let mut res: bool = true;
        for (i, c) in hex_code.chars().enumerate() {
            match self.str_to_hex_map.get(&c) {
                Some(Some(hex)) => res = res && self.compare_nibble(i, &hex),
                Some(None) => res = res && true,
                _ => res = res && false
            }
        }
        res
    }

    fn compare_nibble(&self, pos: usize, nibble: &u8) -> bool{
        match pos {
            0 => *nibble == self.opcode.n1,
            1 => *nibble == self.opcode.n2,
            2 => *nibble == self.opcode.n3,
            3 => *nibble == self.opcode.n4,
            _ => false
        }
    }
}

pub fn run() {
    let mut cpu = CPU::init();
    cpu.ram.init_fonts();
    // load_rom("src/chip_8/roms/ibm_logo.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/BC_test.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Keypad_Test_Hap_2006.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Trip8 Demo (2008) [Revival Studios].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Zero Demo [zeroZshadow, 2007].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Particle Demo [zeroZshadow, 2008].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Sierpinski [Sergey Naydenov, 2010].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Chip8 emulator Logo [Garstyciuks].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/Stars [Sergey Naydenov, 2010].ch8", &mut cpu);
    // load_rom("src/chip_8/roms/test_opcode.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/HIDDEN.ch8", &mut cpu); // Good to test keyboard
    load_rom("src/chip_8/roms/CAVE.ch8", &mut cpu); // Good to test keyboard
    // load_rom("src/chip_8/roms/TRON.ch8", &mut cpu); // Good to test keyboard
    // load_rom("src/chip_8/roms/PUZZLE.ch8", &mut cpu); // Good to test keyboard?
    // load_rom("src/chip_8/roms/TETRIS.ch8", &mut cpu); // Good to test keyboard? 
    // load_rom("src/chip_8/roms/delay_timer_test.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/random_number_test.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/SNAKE.ch8", &mut cpu);

    let sdl_context = sdl2::init().unwrap();
    let mut display = crate::chip_8::display::Display::init(&sdl_context, DISPLAY_SCALE);
    let mut event_listener = sdl_context.event_pump().unwrap();
    cpu.clock.clock_hz = 60;

    'runner: loop {
        for event in event_listener.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'runner,
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {break 'runner},
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::RightBracket), .. } => {
                    println!("Increasing cpu clock from {} Hz to {} Hz", cpu.clock.clock_hz, cpu.clock.clock_hz + 10);
                    cpu.clock.clock_hz += 10;
                },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::LeftBracket), .. } => {
                    println!("Decreasing cpu clock from {} Hz to {} Hz", cpu.clock.clock_hz, cpu.clock.clock_hz - 10);
                    cpu.clock.clock_hz -= 10;
                },
                sdl2::event::Event::KeyDown { 
                    keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => {
                    cpu.pc.set(0x200);
                },
                sdl2::event::Event::KeyDown { keycode: Some(keycode), ..} => {
                    if let Some(key_index) = cpu.keypad.compute_keycode(keycode) {
                        cpu.keypad.press(key_index);
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(keycode), ..} => {
                    if let Some(key_index) = cpu.keypad.compute_keycode(keycode) {
                        cpu.keypad.release(key_index);
                    }
                },
                _ => {}
            }
        }

        cpu.delay_timer.tick();
        cpu.sound_timer.tick();
        if cpu.clock.tick() {
            cpu.fetch();
            cpu.decode();
            if cpu.decode_match("D???") {
                display.draw(&cpu.display_buffer.0)
            }
        }
    }
}

fn load_rom(filename: &str, cpu: &mut CPU) {
    let mut f = std::fs::File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    cpu.ram.load_rom(&buffer);
}

// fn op_0NNN() {}//ignore

fn op_00e0(cpu: &mut CPU) {
    cpu.display_buffer.clear();
}

fn op_1nnn(cpu: &mut CPU) {
    cpu.pc.set(cpu.opcode.nnn);
}

fn op_00ee(cpu: &mut CPU) {
    let value = cpu.stack.pop();
    match value {
        Some(value) => {
            cpu.pc.set(value);
        }
        _ => {}
    }
}

fn op_2nnn(cpu: &mut CPU) {
    cpu.stack.push(cpu.pc.get());
    cpu.pc.set(cpu.opcode.nnn);
}

fn op_3xnn(cpu: &mut CPU) {
    if cpu.registers.get(cpu.opcode.x) == cpu.opcode.nn {
        cpu.pc.increase();
    }
}

fn op_4xnn(cpu: &mut CPU) {
    if cpu.registers.get(cpu.opcode.x) != cpu.opcode.nn {
        cpu.pc.increase();
    }
}

fn op_5xy0(cpu: &mut CPU) {
    if cpu.registers.get(cpu.opcode.x) == cpu.registers.get(cpu.opcode.y) {
        cpu.pc.increase();
    }
}

fn op_9xy0(cpu: &mut CPU) {
    if cpu.registers.get(cpu.opcode.x) != cpu.registers.get(cpu.opcode.y) {
        cpu.pc.increase();
    }
}

fn op_6xnn(cpu: &mut CPU) {
    cpu.registers.set(cpu.opcode.x, cpu.opcode.nn);
}

fn op_7xnn(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    cpu.registers.set(cpu.opcode.x, cpu.opcode.nn.wrapping_add(vx));
}

fn op_8xy0(cpu: &mut CPU) {
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vy);
}

fn op_8xy1(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vx | vy);
}

fn op_8xy2(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vx & vy);
}

fn op_8xy3(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vx ^ vy);
}

fn op_8xy4(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vx.wrapping_add(vy));
}

fn op_8xy5(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vx.wrapping_sub(vy));
    cpu.registers.x_f = if vx > vy {1} else {0};
}

fn op_8xy6(cpu: &mut CPU) {
    let vy = cpu.registers.get(cpu.opcode.x);
    let vx = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vy >> 1);
    cpu.registers.x_f = vx & 0x1;
}

fn op_8xy7(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vy.wrapping_sub(vx));
    cpu.registers.x_f = if vy > vx {1} else {0};
}

fn op_8xye(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    let vy = cpu.registers.get(cpu.opcode.y);
    cpu.registers.set(cpu.opcode.x, vy << 1);
    cpu.registers.x_f = (vx & 0x80) >> 7;
    
}

fn op_annn(cpu: &mut CPU) {
    cpu.index_register = cpu.opcode.nnn;
}

fn op_bnnn(cpu: &mut CPU) {
    cpu.pc.set(cpu.opcode.nnn + cpu.registers.x_0 as usize);
}

fn op_cxnn(cpu: &mut CPU) {
    let mut rng = rand::thread_rng();
    cpu.registers.set(cpu.opcode.x, rng.gen_range(0x0..0xFF) & cpu.opcode.nn);
}

fn op_dxyn(cpu: &mut CPU) {
    let mut vf: bool = false;
    let value = cpu.opcode.n as usize;
    let ori_x = cpu.registers.get(cpu.opcode.x) as usize % DISPLAY_WIDTH;
    let ori_y = cpu.registers.get(cpu.opcode.y) as usize % DISPLAY_HEIGHT;

    for row in 0..value {
        let y = ori_y + row;
        if y >= DISPLAY_HEIGHT {
            break;
        }

        let sprite = cpu.ram.read8(cpu.index_register + row);
        for pixel_position in 0..8 {
            let x = ori_x + pixel_position;
            if x >= DISPLAY_WIDTH {
                break;
            }

            let memory_pixel: bool = (sprite & (1 << (7 - pixel_position))) > 0;
            let display_pixel: bool = cpu.display_buffer.0[x][y];
            cpu.display_buffer.0[x][y] = memory_pixel ^ display_pixel;
            vf = (memory_pixel && display_pixel) || vf;
        }
    }
    cpu.registers.x_f = if vf {1} else {0};
}

fn op_ex9e(cpu: &mut CPU) {
    if cpu.keypad.get(cpu.registers.get(cpu.opcode.x) as usize) {
        cpu.pc.increase();
    }
}

fn op_exa1(cpu: &mut CPU) {
    if !cpu.keypad.get(cpu.registers.get(cpu.opcode.x) as usize) {
        cpu.pc.increase();
    }
}

fn op_fx07(cpu: &mut CPU) {
    cpu.registers.set(cpu.opcode.x, cpu.delay_timer.tick);
}

fn op_fx15(cpu: &mut CPU) {
    cpu.delay_timer.tick = cpu.registers.get(cpu.opcode.x);
}

fn op_fx18(cpu: &mut CPU) {
    cpu.sound_timer.tick = cpu.registers.get(cpu.opcode.x);
}

fn op_fx1e(cpu: &mut CPU) {
    cpu.index_register += cpu.registers.get(cpu.opcode.x) as usize;
}

fn op_fx0a(cpu: &mut CPU) {
    match cpu.keypad.being_pressed() {
        Some(key) => { cpu.registers.set(cpu.opcode.x, key); },
        _ => { cpu.pc.decrement(); }
    }
}

fn op_fx29(cpu: &mut CPU) {
    let char = (cpu.registers.get(cpu.opcode.x) & 0xF) as usize;
    cpu.index_register = 0x50 + char * 5;
}

fn op_fx33(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.opcode.x);
    cpu.ram.write8(cpu.index_register, vx / 100);
    cpu.ram.write8(cpu.index_register + 1, vx / 10 % 10);
    cpu.ram.write8(cpu.index_register + 2, vx % 10);
}

fn op_fx55(cpu: &mut CPU) {
    let i = cpu.index_register;
    for regs in 0x0..(cpu.opcode.x + 1) {
        cpu.ram.write8(i + regs, cpu.registers.get(regs));
    }
}

fn op_fx65(cpu: &mut CPU) {
    let i = cpu.index_register;
    for regs in 0x0..(cpu.opcode.x + 1) {
        cpu.registers.set(regs, cpu.ram.read8(i + regs));
    }
}