// CHIP-8 means Compact Hexadecimal Interpretive Programming - 8-bit
use ::sdl2;

use rand::Rng;

use std::io::Read;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SCALE: u32 = 10;

pub struct ProgramCounter(usize);
pub struct Registers([u8; 0x10]);
pub struct Ram([u8; 4 * 1024]);
pub struct DisplayBuffer([[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH]);

pub struct Timer{
    tick: u8,
    clock_hz: u128,
    elapsed: std::time::SystemTime
}

pub struct Keypad { 
    keys: [bool; 0x10], 
    // layout: String 
}

pub struct OpCodes { 
    opcode: u16, 
    nible_1: u8, 
    nible_2: u8, 
    nible_3: u8, 
    nible_4: u8 
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
            0: [0x0; 16]
        }
    }

    fn get(&self, register: usize) -> u8 {
        self.0[register]
    }

    fn set(&mut self, register: usize, value: u8) {
        self.0[register] = value;
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
            keys: [false; 0x10],
            // layout: String::from("123C456D789EA0BF")
        }
    }

    fn compute_keycode(&self, keycode: sdl2::keyboard::Keycode) -> Option<usize> {
        match keycode {
            sdl2::keyboard::Keycode::Num1 => Some(0x1),
            sdl2::keyboard::Keycode::Num2 => Some(0x2),
            sdl2::keyboard::Keycode::Num3 => Some(0x3),
            sdl2::keyboard::Keycode::Num4 => Some(0xC),

            sdl2::keyboard::Keycode::Q => Some(0x4),
            sdl2::keyboard::Keycode::W => Some(0x5),
            sdl2::keyboard::Keycode::E => Some(0x6),
            sdl2::keyboard::Keycode::R => Some(0xD),

            sdl2::keyboard::Keycode::A => Some(0x7),
            sdl2::keyboard::Keycode::S => Some(0x8),
            sdl2::keyboard::Keycode::D => Some(0x9),
            sdl2::keyboard::Keycode::F => Some(0xE),

            sdl2::keyboard::Keycode::Z => Some(0xA),
            sdl2::keyboard::Keycode::X => Some(0x0),
            sdl2::keyboard::Keycode::C => Some(0xB),
            sdl2::keyboard::Keycode::V => Some(0xF),

            _ => Option::None,
        }
    }

    // fn set_layout(&mut self, layout: &str) {
    //     assert_eq!(layout.len(), self.layout.len());
    //     self.layout = layout.to_string();
    // }

    fn get(&mut self, pos: usize) -> bool {
        self.keys[pos]
    }

    fn being_pressed(&self) -> u8 {
        for key in 0x0..0x10 {
            if self.keys[key] {
                return key as u8
            }
        }
        return 0x10
    }

    fn press(&mut self, key: usize) {
        self.keys[key] = true;
    }

    fn release(&mut self, key: usize) {
        self.keys[key] = false;
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
        OpCodes {
            opcode: opcode,
            nible_1: ((opcode & 0xF000) >> 12) as u8,
            nible_2: ((opcode & 0x0F00) >> 8) as u8,
            nible_3: ((opcode & 0x00F0) >> 4) as u8,
            nible_4: (opcode & 0x000F) as u8
        }
    }

    fn get_x(&self) -> usize {
        self.nible_2 as usize
    }

    fn get_y(&self) -> usize {
        self.nible_3 as usize
    }

    fn get_nnn(&self) -> u16 {
        self.opcode & 0xFFF
    }

    fn get_nn(&self) -> u8 {
        (self.opcode & 0xFF) as u8
    }

    fn get_n(&self) -> u8 {
        (self.opcode & 0xF) as  u8
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
    display_buffer: DisplayBuffer
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
            display_buffer: DisplayBuffer::init()
        }
    }
}

pub fn run() {
    let mut cpu = CPU::init();
    cpu.ram.init_fonts();
    // load_rom("src/chip_8/roms/ibm_logo.ch8", &mut cpu);
    load_rom("src/chip_8/roms/BC_test.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/test_opcode.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/HIDDEN.ch8", &mut cpu); // Good to test keyboard
    // load_rom("src/chip_8/roms/CAVE.ch8", &mut cpu); // Good to test keyboard
    // load_rom("src/chip_8/roms/TRON.ch8", &mut cpu); // Good to test keyboard
    // load_rom("src/chip_8/roms/PUZZLE.ch8", &mut cpu); // Good to test keyboard?
    // load_rom("src/chip_8/roms/TETRIS.ch8", &mut cpu); // Good to test keyboard? 
    // load_rom("src/chip_8/roms/delay_timer_test.ch8", &mut cpu);
    // load_rom("src/chip_8/roms/random_number_test.ch8", &mut cpu);

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
            let opcode = OpCodes::init(fetch(&mut cpu));
            decode(&opcode, &mut cpu);
            if opcode.nible_1 == 0xD {
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

fn fetch(cpu: &mut CPU) -> u16 {
    let opcode = cpu.ram.read16(cpu.pc.get());
    cpu.pc.increase();
    opcode
}

fn decode(opcode: &OpCodes, cpu: &mut CPU) {
    if opcode.opcode == 0x00E0 {
        op_00e0(cpu);
    } else if opcode.nible_1 == 0x1 {
        op1nnn(cpu, opcode.get_nnn() as usize);
    } else if opcode.opcode == 0x00EE {
        op00ee(cpu);
    } else if opcode.nible_1 == 0x2 {
        op2nnn(cpu, opcode.get_nnn() as usize);
    } else if opcode.nible_1 == 0x3 {
        op3xnn(cpu, opcode.get_x(), opcode.get_nn());
    } else if opcode.nible_1 == 0x4 {
        op4xnn(cpu, opcode.get_x(), opcode.get_nn());
    } else if opcode.nible_1 == 0x5 {
        op5xy0(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x9 {
        op9xy0(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x6 {
        op6xnn(cpu, opcode.get_x(), opcode.get_nn());
    } else if opcode.nible_1 == 0x7 {
        op7xnn(cpu, opcode.get_x(), opcode.get_nn());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x0 {
        op8xy0(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x1 {
        op8xy1(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x2 {
        op8xy2(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x3 {
        op8xy3(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x4 {
        op8xy4(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x5 {
        op8xy5(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x6 {
        op8xy6(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0x7 {
        op8xy7(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0x8 && opcode.nible_4 == 0xE {
        op8xye(cpu, opcode.get_x(), opcode.get_y());
    } else if opcode.nible_1 == 0xA {
        opannn(cpu, opcode.get_nnn() as usize);
    } else if opcode.nible_1 == 0xB {
        opbnnn(cpu, opcode.get_nnn() as usize);
    } else if opcode.nible_1 == 0xC {
        opcxnn(cpu, opcode.get_x(), opcode.get_nn());
    } else if opcode.nible_1 == 0xD {
        opdxyn(cpu, opcode.get_x(), opcode.get_y(), opcode.get_n());
    } else if opcode.nible_1 == 0xE && opcode.nible_3 == 0x9 && opcode.nible_4 == 0xE {
        opex9e(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xE && opcode.nible_3 == 0x9 && opcode.nible_4 == 0xE {
        opex9e(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xE && opcode.nible_3 == 0xA && opcode.nible_4 == 0x1 {
        opexa1(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x0 && opcode.nible_4 == 0x7 {
        opfx07(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x1 && opcode.nible_4 == 0x5 {
        opfx15(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x1 && opcode.nible_4 == 0x8 {
        opfx18(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x1 && opcode.nible_4 == 0xE {
        opfx1e(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x0 && opcode.nible_4 == 0xA {
        opfx0a(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x2 && opcode.nible_4 == 0x9 {
        opfx29(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x3 && opcode.nible_4 == 0x3 {
        opfx33(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x5 && opcode.nible_4 == 0x5 {
        opfx55(cpu, opcode.get_x());
    } else if opcode.nible_1 == 0xF && opcode.nible_3 == 0x6 && opcode.nible_4 == 0x5 {
        opfx65(cpu, opcode.get_x());
    } else {
        println!{"Unknown instruction: {:04x}", opcode.opcode};
    }
}
// fn execute() {}

// fn op_0NNN() {}//ignore

// CLS
// Clear the display, turning all pixels off
fn op_00e0(cpu: &mut CPU) {
    cpu.display_buffer.clear();
}

// JMP
// Set PC to NNN, causing the program to jump to that memory location
fn op1nnn(cpu: &mut CPU, value: usize) {
    cpu.pc.set(value);
}

fn op00ee(cpu: &mut CPU) {
    let value = cpu.stack.pop();
    match value {
        Some(value) => {
            cpu.pc.set(value);
        }
        _ => {}
    }
}

fn op2nnn(cpu: &mut CPU, value: usize) {
    cpu.stack.push(cpu.pc.get());
    cpu.pc.set(value);
}

fn op3xnn(cpu: &mut CPU, register_x: usize, value: u8) {
    if cpu.registers.get(register_x) == value {
        cpu.pc.increase();
    }
}

fn op4xnn(cpu: &mut CPU, register_x: usize, value: u8) {
    if cpu.registers.get(register_x) != value {
        cpu.pc.increase();
    }
}

fn op5xy0(cpu: &mut CPU, register_x: usize, register_y: usize) {
    if cpu.registers.get(register_x) == cpu.registers.get(register_y) {
        cpu.pc.increase();
    }
}

fn op9xy0(cpu: &mut CPU, register_x: usize, register_y: usize) {
    if cpu.registers.get(register_x) != cpu.registers.get(register_y) {
        cpu.pc.increase();
    }
}

// SET
// Set the register VX to the value NN
fn op6xnn(cpu: &mut CPU, register: usize, value: u8) {
    cpu.registers.set(register, value);
}

// ADD
// Add the value NN to VX. VF is ignored
fn op7xnn(cpu: &mut CPU, register: usize, value: u8) {
    let nn = value as u16;
    let vx = cpu.registers.get(register) as u16;
    let value = (nn + vx) & 0xFF;
    let value = value as u8;
    cpu.registers.set(register, value);
}

fn op8xy0(cpu: &mut CPU, register_x: usize, register_y: usize) {
    cpu.registers.set(register_x, cpu.registers.get(register_y));
}

fn op8xy1(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x);
    let vy = cpu.registers.get(register_y);
    cpu.registers.set(register_x, vx | vy);
}

fn op8xy2(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x);
    let vy = cpu.registers.get(register_y);
    cpu.registers.set(register_x, vx & vy);
}

fn op8xy3(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x);
    let vy = cpu.registers.get(register_y);
    cpu.registers.set(register_x, vx ^ vy);
}

fn op8xy4(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x) as u16;
    let vy = cpu.registers.get(register_y) as u16;
    let value = (vx + vy) & 0xFF;
    cpu.registers.set(register_x, value as u8);
}

fn op8xy5(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x);
    let vy = cpu.registers.get(register_y);
    cpu.registers.set(register_x, vx.wrapping_sub(vy));
    cpu.registers.set(0xF, if vx > vy {1} else {0});
}

fn op8xy6(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vy = cpu.registers.get(register_y);
    let vx = cpu.registers.get(register_x);
    cpu.registers.set(register_x, vy >> 1);
    cpu.registers.set(0xF, vx & 0x1);
}

fn op8xy7(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vx = cpu.registers.get(register_x);
    let vy = cpu.registers.get(register_y);
    cpu.registers.set(register_x, vy.wrapping_sub(vx));
    cpu.registers.set(0xF, if vy > vx {1} else {0});
}

fn op8xye(cpu: &mut CPU, register_x: usize, register_y: usize) {
    let vy = cpu.registers.get(register_y);
    let vx = cpu.registers.get(register_x);
    cpu.registers.set(register_x, vy << 1);
    cpu.registers.set(0xF, (vx & 0x80) >> 7);
    
}

// IND
// Set index register I to the value NNN
fn opannn(cpu: &mut CPU, value: usize) {
    cpu.index_register = value;
}

fn opbnnn(cpu: &mut CPU, value: usize) {
    let v0 = cpu.registers.get(0x0) as usize;
    cpu.pc.set(value + v0);
}

fn opcxnn(cpu: &mut CPU, register_x: usize, value: u8) {
    let mut rng = rand::thread_rng();
    cpu.registers.set(register_x,rng.gen_range(0x0..0xFF) & value);
}

// DIS
// Display
fn opdxyn(cpu: &mut CPU, register_x: usize, register_y: usize, value: u8) {
    let mut vf: bool = false;
    let value = value as usize;
    let ori_x = cpu.registers.get(register_x) as usize % DISPLAY_WIDTH;
    let ori_y = cpu.registers.get(register_y) as usize % DISPLAY_HEIGHT;

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
    cpu.registers.set(0xF, if vf {1} else {0});
}

fn opex9e(cpu: &mut CPU, register_x: usize) {
    if cpu.keypad.get(cpu.registers.get(register_x) as usize) {
        cpu.pc.increase();
    }
}

fn opexa1(cpu: &mut CPU, register_x: usize) {
    if !cpu.keypad.get(cpu.registers.get(register_x) as usize) {
        cpu.pc.increase();
    }
}

fn opfx07(cpu: &mut CPU, register_x: usize) {
    cpu.registers.set(register_x, cpu.delay_timer.tick);
}

fn opfx15(cpu: &mut CPU, register_x: usize) {
    cpu.delay_timer.tick = cpu.registers.get(register_x);
}

fn opfx18(cpu: &mut CPU, register_x: usize) {
    cpu.sound_timer.tick = cpu.registers.get(register_x);
}

fn opfx1e(cpu: &mut CPU, register_x: usize) {
    cpu.index_register += cpu.registers.get(register_x) as usize;
}

fn opfx0a(cpu: &mut CPU, register_x: usize) {
    if cpu.keypad.being_pressed() == 0x10 {
        cpu.pc.decrement();
    } else {
        cpu.registers.set(register_x, cpu.keypad.being_pressed());
    }
}

fn opfx29(cpu: &mut CPU, register_x: usize) {
    let char = (cpu.registers.get(register_x) & 0xF) as usize;
    cpu.index_register = 0x50 + char * 5;
}

fn opfx33(cpu: &mut CPU, register_x: usize) {
    let vx = cpu.registers.get(register_x);
    cpu.ram.write8(cpu.index_register, vx / 100);
    cpu.ram.write8(cpu.index_register + 1, vx / 10 % 10);
    cpu.ram.write8(cpu.index_register + 2, vx % 10);
}

fn opfx55(cpu: &mut CPU, register_x: usize) {
    let i = cpu.index_register;
    for regs in 0x0..(register_x + 1) {
        cpu.ram.write8(i + regs, cpu.registers.get(regs));
    }
    // cpu.index_register += register_x + 1;
}

fn opfx65(cpu: &mut CPU, register_x: usize) {
    let i = cpu.index_register;
    for regs in 0x0..(register_x + 1) {
        cpu.registers.set(regs, cpu.ram.read8(i + regs));
    }
    // cpu.index_register += register_x + 1;
}