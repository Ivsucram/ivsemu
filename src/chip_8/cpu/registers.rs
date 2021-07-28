const INITIAL_PC: usize = 0x200;

pub struct Registers {
    x_0: u8,
    x_1: u8,
    x_2: u8,
    x_3: u8,
    x_4: u8,
    x_5: u8,
    x_6: u8,
    x_7: u8,
    x_8: u8,
    x_9: u8,
    x_a: u8,
    x_b: u8,
    x_c: u8,
    x_d: u8,
    x_e: u8,
    x_f: u8,        // Flag register
    pub pc: usize,  // Program Counter
    pub i: usize    // Index register
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            x_0: 0,
            x_1: 0,
            x_2: 0,
            x_3: 0,
            x_4: 0,
            x_5: 0,
            x_6: 0,
            x_7: 0,
            x_8: 0,
            x_9: 0,
            x_a: 0,
            x_b: 0,
            x_c: 0,
            x_d: 0,
            x_e: 0,
            x_f: 0,
            pc: INITIAL_PC,
            i: 0,
        }
    }

    pub fn get(&self, register: usize) -> u8 {
        assert!(register <= 0xF);
        match register {
            0x0 => self.x_0,
            0x1 => self.x_1,
            0x2 => self.x_2,
            0x3 => self.x_3,
            0x4 => self.x_4,
            0x5 => self.x_5,
            0x6 => self.x_6,
            0x7 => self.x_7,
            0x8 => self.x_8,
            0x9 => self.x_9,
            0xA => self.x_a,
            0xB => self.x_b,
            0xC => self.x_c,
            0xD => self.x_d,
            0xE => self.x_e,
            0xF => self.x_f,
            _ => 0,
        }
    }

    pub fn set(&mut self, register: usize, value: u8) {
        assert!(register <= 0xF);
        match register {
            0x0 => self.x_0 = value,
            0x1 => self.x_1 = value,
            0x2 => self.x_2 = value,
            0x3 => self.x_3 = value,
            0x4 => self.x_4 = value,
            0x5 => self.x_5 = value,
            0x6 => self.x_6 = value,
            0x7 => self.x_7 = value,
            0x8 => self.x_8 = value,
            0x9 => self.x_9 = value,
            0xA => self.x_a = value,
            0xB => self.x_b = value,
            0xC => self.x_c = value,
            0xD => self.x_d = value,
            0xE => self.x_e = value,
            0xF => self.x_f = value,
            _ => {}
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn decrement_pc(&mut self) {
        self.pc -= 2;
    }

    pub fn reset_pc(&mut self) {
        self.pc = INITIAL_PC;
    }
}
