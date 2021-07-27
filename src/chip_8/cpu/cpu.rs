use super::clock::Clock;
use super::display_buffer::{DisplayBuffer, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use super::keypad::Keypad;
use super::opcodes::OpCodes;
use super::program_counter::ProgramCounter;
use super::ram::RAM;
use super::registers::Registers;

use rand::Rng;

pub struct CPU {
    pub pc: ProgramCounter,
    pub i: usize,
    pub stack: Vec<usize>,
    pub dt: Clock,
    pub st: Clock,
    pub clock: Clock,
    pub registers: Registers,
    pub ram: RAM,
    pub keypad: Keypad,
    pub db: DisplayBuffer,
    pub op: OpCodes,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: ProgramCounter::new(),
            i: 0,
            stack: vec![],
            dt: Clock::new(),
            st: Clock::new(),
            clock: Clock::new(),
            registers: Registers::new(),
            ram: RAM::new(),
            keypad: Keypad::new(),
            db: DisplayBuffer::new(10),
            op: OpCodes::new(0000),
        }
    }

    pub fn fetch(&mut self) {
        self.op = OpCodes::new(self.ram.read16(self.pc.0));
        self.pc.increment();
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

    pub fn decode_match(&self, hex_code: &str) -> bool {
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
}

fn op_00e0(cpu: &mut CPU) {
    cpu.db.clear();
}

fn op_1nnn(cpu: &mut CPU) {
    cpu.pc.0 = cpu.op.nnn;
}

fn op_00ee(cpu: &mut CPU) {
    let value = cpu.stack.pop();
    match value {
        Some(value) => {
            cpu.pc.0 = value;
        }
        _ => {}
    }
}

fn op_2nnn(cpu: &mut CPU) {
    cpu.stack.push(cpu.pc.0);
    cpu.pc.0 = cpu.op.nnn;
}

fn op_3xnn(cpu: &mut CPU) {
    if cpu.registers.get(cpu.op.x) == cpu.op.nn {
        cpu.pc.increment();
    }
}

fn op_4xnn(cpu: &mut CPU) {
    if cpu.registers.get(cpu.op.x) != cpu.op.nn {
        cpu.pc.increment();
    }
}

fn op_5xy0(cpu: &mut CPU) {
    if cpu.registers.get(cpu.op.x) == cpu.registers.get(cpu.op.y) {
        cpu.pc.increment();
    }
}

fn op_9xy0(cpu: &mut CPU) {
    if cpu.registers.get(cpu.op.x) != cpu.registers.get(cpu.op.y) {
        cpu.pc.increment();
    }
}

fn op_6xnn(cpu: &mut CPU) {
    cpu.registers.set(cpu.op.x, cpu.op.nn);
}

fn op_7xnn(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    cpu.registers.set(cpu.op.x, cpu.op.nn.wrapping_add(vx));
}

fn op_8xy0(cpu: &mut CPU) {
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vy);
}

fn op_8xy1(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vx | vy);
}

fn op_8xy2(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vx & vy);
}

fn op_8xy3(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vx ^ vy);
}

fn op_8xy4(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vx.wrapping_add(vy));
}

fn op_8xy5(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vx.wrapping_sub(vy));
    cpu.registers.x_f = if vx > vy { 1 } else { 0 };
}

fn op_8xy6(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vy >> 1);
    cpu.registers.x_f = vx & 0x1;
}

fn op_8xy7(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vy.wrapping_sub(vx));
    cpu.registers.x_f = if vy > vx { 1 } else { 0 };
}

fn op_8xye(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    let vy = cpu.registers.get(cpu.op.y);
    cpu.registers.set(cpu.op.x, vy << 1);
    cpu.registers.x_f = (vx & 0x80) >> 7;
}

fn op_annn(cpu: &mut CPU) {
    cpu.i = cpu.op.nnn;
}

fn op_bnnn(cpu: &mut CPU) {
    cpu.pc.0 = cpu.op.nnn + cpu.registers.x_0 as usize;
}

fn op_cxnn(cpu: &mut CPU) {
    let mut rng = rand::thread_rng();
    cpu.registers
        .set(cpu.op.x, rng.gen_range(0x0..0xFF) & cpu.op.nn);
}

fn op_dxyn(cpu: &mut CPU) {
    let mut vf: bool = false;
    let value = cpu.op.n as usize;
    let ori_x = cpu.registers.get(cpu.op.x) as usize % DISPLAY_WIDTH;
    let ori_y = cpu.registers.get(cpu.op.y) as usize % DISPLAY_HEIGHT;

    for row in 0..value {
        let y = ori_y + row;
        if y >= DISPLAY_HEIGHT {
            break;
        }

        let sprite = cpu.ram.read8(cpu.i + row);
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
    cpu.registers.x_f = if vf { 1 } else { 0 };
}

fn op_ex9e(cpu: &mut CPU) {
    if cpu.keypad.get(cpu.registers.get(cpu.op.x) as usize) {
        cpu.pc.increment();
    }
}

fn op_exa1(cpu: &mut CPU) {
    if !cpu.keypad.get(cpu.registers.get(cpu.op.x) as usize) {
        cpu.pc.increment();
    }
}

fn op_fx07(cpu: &mut CPU) {
    cpu.registers.set(cpu.op.x, cpu.dt.tick);
}

fn op_fx15(cpu: &mut CPU) {
    cpu.dt.tick = cpu.registers.get(cpu.op.x);
}

fn op_fx18(cpu: &mut CPU) {
    cpu.st.tick = cpu.registers.get(cpu.op.x);
}

fn op_fx1e(cpu: &mut CPU) {
    cpu.i += cpu.registers.get(cpu.op.x) as usize;
}

fn op_fx0a(cpu: &mut CPU) {
    match cpu.keypad.being_pressed() {
        Some(key) => {
            cpu.registers.set(cpu.op.x, key);
        }
        _ => {
            cpu.pc.decrement();
        }
    }
}

fn op_fx29(cpu: &mut CPU) {
    let char = (cpu.registers.get(cpu.op.x) & 0xF) as usize;
    cpu.i = cpu.ram.font_address + char * 5;
}

fn op_fx33(cpu: &mut CPU) {
    let vx = cpu.registers.get(cpu.op.x);
    cpu.ram.write(cpu.i, vx / 100);
    cpu.ram.write(cpu.i + 1, vx / 10 % 10);
    cpu.ram.write(cpu.i + 2, vx % 10);
}

fn op_fx55(cpu: &mut CPU) {
    let i = cpu.i;
    for regs in 0x0..(cpu.op.x + 1) {
        cpu.ram.write(i + regs, cpu.registers.get(regs));
    }
}

fn op_fx65(cpu: &mut CPU) {
    let i = cpu.i;
    for regs in 0x0..(cpu.op.x + 1) {
        cpu.registers.set(regs, cpu.ram.read8(i + regs));
    }
}
