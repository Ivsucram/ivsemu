use super::clock::Clock;
use super::display_buffer::{DisplayBuffer, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use super::keypad::Keypad;
use super::opcodes::OpCodes;
use super::ram::RAM;
use super::registers::Registers;

use sdl2::keyboard::Keycode;

use rand::Rng;

pub struct CPU {
    stack: Vec<usize>,      // Function Stack
    dt: Clock,              // Delay Timer
    st: Clock,              // Sound Timer
    clock: Clock,           // CPU Clock
    regs: Registers,        // Registers
    ram: RAM,               // RAM
    keypad: Keypad,         // Keypad
    db: DisplayBuffer,      // Display Buffer
    op: OpCodes,            // Operation Code,
    pub should_redraw: bool // Boolean indicating Display Buffer update
}

impl CPU {
    pub fn new() -> CPU {
        let mut ram = RAM::new();
        ram.init_fonts();

        CPU {
            stack: vec![],
            dt: Clock::new(),
            st: Clock::new(),
            clock: Clock::new(),
            regs: Registers::new(),
            ram: ram,
            keypad: Keypad::new(),
            db: DisplayBuffer::new(10),
            op: OpCodes::new(0000),
            should_redraw: false
        }
    }

    pub fn fetch(&mut self) {
        self.op = OpCodes::new(self.ram.read16(self.regs.pc));
        self.regs.increment_pc();
    }

    pub fn decode(&mut self) {
        //TODO: function pointers
        if self.decode_match("00E0") {
            op_00e0(self);
        } else if self.decode_match("1???") {
            op_1nnn(self);
        } else if self.decode_match("00EE") {
            op_00ee(self);
        } else if self.decode_match("2???") {
            op_2nnn(self);
        } else if self.decode_match("3???") {
            op_3xnn(self);
        } else if self.decode_match("4???") {
            op_4xnn(self);
        } else if self.decode_match("5??0") {
            op_5xy0(self);
        } else if self.decode_match("9??0") {
            op_9xy0(self);
        } else if self.decode_match("6???") {
            op_6xnn(self);
        } else if self.decode_match("7???") {
            op_7xnn(self);
        } else if self.decode_match("8??0") {
            op_8xy0(self);
        } else if self.decode_match("8??1") {
            op_8xy1(self);
        } else if self.decode_match("8??2") {
            op_8xy2(self);
        } else if self.decode_match("8??3") {
            op_8xy3(self);
        } else if self.decode_match("8??4") {
            op_8xy4(self);
        } else if self.decode_match("8??5") {
            op_8xy5(self);
        } else if self.decode_match("8??6") {
            op_8xy6(self);
        } else if self.decode_match("8??7") {
            op_8xy7(self);
        } else if self.decode_match("8??E") {
            op_8xye(self);
        } else if self.decode_match("A???") {
            op_annn(self);
        } else if self.decode_match("B???") {
            op_bnnn(self);
        } else if self.decode_match("C???") {
            op_cxnn(self);
        } else if self.decode_match("D???") {
            op_dxyn(self);
            self.should_redraw = true;
        } else if self.decode_match("E?9E") {
            op_ex9e(self);
        } else if self.decode_match("E?A1") {
            op_exa1(self);
        } else if self.decode_match("F??7") {
            op_fx07(self);
        } else if self.decode_match("F?15") {
            op_fx15(self);
        } else if self.decode_match("F?18") {
            op_fx18(self);
        } else if self.decode_match("F?1E") {
            op_fx1e(self);
        } else if self.decode_match("F?0A") {
            op_fx0a(self);
        } else if self.decode_match("F?29") {
            op_fx29(self);
        } else if self.decode_match("F?33") {
            op_fx33(self);
        } else if self.decode_match("F?55") {
            op_fx55(self);
        } else if self.decode_match("F?65") {
            op_fx65(self);
        } else {
            println! {"Unknown instruction: {:04x}", self.op.opcode};
        }
    }

    fn decode_match(&self, hex_code: &str) -> bool {
        assert!(
            hex_code.len() == 4,
            "Instruction with wrong size. All chip-8 instructions have 16 bits"
        );
        assert!(hex_code.ne("????"), "???? is a invalid instruction");
        let mut res: bool = true;
        for (i, c) in hex_code.chars().enumerate() {
            match self.op.str_to_hex_helper.get(&c) {
                Some(None) => {
                    res = true;
                }
                Some(Some(hex)) => {
                    if !self.compare_nibble(i, &hex) {
                        res = false;
                        break;
                    }
                }
                _ => {
                    res = false;
                    break;
                }
            }
        }
        res
    }

    fn compare_nibble(&self, pos: usize, nibble: &u8) -> bool {
        match pos {
            0 => *nibble == self.op.n1,
            1 => *nibble == self.op.n2,
            2 => *nibble == self.op.n3,
            3 => *nibble == self.op.n4,
            _ => false,
        }
    }

    pub fn reset_rom(&mut self) {
        self.regs.reset_pc();
    }

    pub fn increase_clock(&mut self, is_printing: bool) {
        self.clock.increase_clock(is_printing);
    }

    pub fn decrease_clock(&mut self, is_printing: bool) {
        self.clock.decrease_clock(is_printing);
    }

    pub fn compute_keycode(&mut self, keycode: Keycode) -> Option<usize> {
        self.keypad.compute_keycode(keycode)
    }

    pub fn press_key(&mut self, key_index: usize) {
        self.keypad.press(key_index);
    }

    pub fn release_key(&mut self, key_index: usize) {
        self.keypad.release(key_index);
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.dt.tick
    }

    pub fn set_delay_timer(&mut self, tick: u8) {
        self.dt.tick = tick;
    } 

    pub fn set_sound_timer(&mut self, tick: u8) {
        self.st.tick = tick;
    }

    pub fn tick_delay_timer(&mut self) -> bool {
        self.dt.tick()
    }

    pub fn tick_sound_timer(&mut self) -> bool {
        self.st.tick()
    }

    pub fn tick(&mut self) -> bool {
        self.clock.tick()
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram.load_rom(rom);
    }

    pub fn get_display_scale(&self) -> u32 {
        self.db.scale
    }

    pub fn get_display_buffer(&self) -> [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] {
        self.db.db
    }
}

fn op_00e0(cpu: &mut CPU) {
    cpu.db.clear();
}

fn op_1nnn(cpu: &mut CPU) {
    cpu.regs.pc = cpu.op.nnn;
}

fn op_00ee(cpu: &mut CPU) {
    let value = cpu.stack.pop();
    match value {
        Some(value) => {
            cpu.regs.pc = value;
        }
        _ => {}
    }
}

fn op_2nnn(cpu: &mut CPU) {
    cpu.stack.push(cpu.regs.pc);
    cpu.regs.pc = cpu.op.nnn;
}

fn op_3xnn(cpu: &mut CPU) {
    if cpu.regs.get(cpu.op.x) == cpu.op.nn {
        cpu.regs.increment_pc();
    }
}

fn op_4xnn(cpu: &mut CPU) {
    if cpu.regs.get(cpu.op.x) != cpu.op.nn {
        cpu.regs.increment_pc();
    }
}

fn op_5xy0(cpu: &mut CPU) {
    if cpu.regs.get(cpu.op.x) == cpu.regs.get(cpu.op.y) {
        cpu.regs.increment_pc();
    }
}

fn op_9xy0(cpu: &mut CPU) {
    if cpu.regs.get(cpu.op.x) != cpu.regs.get(cpu.op.y) {
        cpu.regs.increment_pc();
    }
}

fn op_6xnn(cpu: &mut CPU) {
    cpu.regs.set(cpu.op.x, cpu.op.nn);
}

fn op_7xnn(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    cpu.regs.set(cpu.op.x, cpu.op.nn.wrapping_add(vx));
}

fn op_8xy0(cpu: &mut CPU) {
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vy);
}

fn op_8xy1(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vx | vy);
}

fn op_8xy2(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vx & vy);
}

fn op_8xy3(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vx ^ vy);
}

fn op_8xy4(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vx.wrapping_add(vy));
}

fn op_8xy5(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vx.wrapping_sub(vy));
    cpu.regs.set(0xF, if vx > vy { 1 } else { 0 });
}

fn op_8xy6(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vy >> 1);
    cpu.regs.set(0xF, vx & 0x1);
}

fn op_8xy7(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vy.wrapping_sub(vx));
    cpu.regs.set(0xF, if vy > vx { 1 } else { 0 });
}

fn op_8xye(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    let vy = cpu.regs.get(cpu.op.y);
    cpu.regs.set(cpu.op.x, vy << 1);
    cpu.regs.set(0xF, (vx & 0x80) >> 7);
}

fn op_annn(cpu: &mut CPU) {
    cpu.regs.i = cpu.op.nnn;
}

fn op_bnnn(cpu: &mut CPU) {
    cpu.regs.pc = cpu.op.nnn + cpu.regs.get(0x0) as usize;
}

fn op_cxnn(cpu: &mut CPU) {
    let mut rng = rand::thread_rng();
    cpu.regs
        .set(cpu.op.x, rng.gen_range(0x0..0xFF) & cpu.op.nn);
}

fn op_dxyn(cpu: &mut CPU) {
    let mut vf: bool = false;
    let value = cpu.op.n as usize;
    let ori_x = cpu.regs.get(cpu.op.x) as usize % DISPLAY_WIDTH;
    let ori_y = cpu.regs.get(cpu.op.y) as usize % DISPLAY_HEIGHT;

    for row in 0..value {
        let y = ori_y + row;
        if y >= DISPLAY_HEIGHT {
            break;
        }

        let sprite = cpu.ram.read8(cpu.regs.i + row);
        for pixel_position in 0..8 {
            let x = ori_x + pixel_position;
            if x >= DISPLAY_WIDTH {
                break;
            }

            let memory_pixel: bool = (sprite & (1 << (7 - pixel_position))) > 0;
            let display_pixel: bool = cpu.db.db[x][y];
            cpu.db.db[x][y] = memory_pixel ^ display_pixel;
            vf = (memory_pixel && display_pixel) || vf;
        }
    }
    cpu.regs.set(0xF, if vf { 1 } else { 0 });
}

fn op_ex9e(cpu: &mut CPU) {
    if cpu.keypad.get_status(cpu.regs.get(cpu.op.x) as usize) {
        cpu.regs.increment_pc();
    }
}

fn op_exa1(cpu: &mut CPU) {
    if !cpu.keypad.get_status(cpu.regs.get(cpu.op.x) as usize) {
        cpu.regs.increment_pc();
    }
}

fn op_fx07(cpu: &mut CPU) {
    cpu.regs.set(cpu.op.x, cpu.get_delay_timer());
}

fn op_fx15(cpu: &mut CPU) {
    cpu.set_delay_timer(cpu.regs.get(cpu.op.x));
}

fn op_fx18(cpu: &mut CPU) {
    cpu.set_sound_timer(cpu.regs.get(cpu.op.x));
}

fn op_fx1e(cpu: &mut CPU) {
    cpu.regs.i += cpu.regs.get(cpu.op.x) as usize;
}

fn op_fx0a(cpu: &mut CPU) {
    match cpu.keypad.being_pressed() {
        Some(key) => {
            cpu.regs.set(cpu.op.x, key);
        }
        _ => {
            cpu.regs.decrement_pc();
        }
    }
}

fn op_fx29(cpu: &mut CPU) {
    let char = (cpu.regs.get(cpu.op.x) & 0xF) as usize;
    cpu.regs.i = cpu.ram.get_font_address() + char * 5;
}

fn op_fx33(cpu: &mut CPU) {
    let vx = cpu.regs.get(cpu.op.x);
    cpu.ram.write8(cpu.regs.i, vx / 100);
    cpu.ram.write8(cpu.regs.i + 1, vx / 10 % 10);
    cpu.ram.write8(cpu.regs.i + 2, vx % 10);
}

fn op_fx55(cpu: &mut CPU) {
    let i = cpu.regs.i;
    for regs in 0x0..(cpu.op.x + 1) {
        cpu.ram.write8(i + regs, cpu.regs.get(regs));
    }
}

fn op_fx65(cpu: &mut CPU) {
    let i = cpu.regs.i;
    for regs in 0x0..(cpu.op.x + 1) {
        cpu.regs.set(regs, cpu.ram.read8(i + regs));
    }
}