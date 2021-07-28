use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct Keypad {
    pub key_status: [bool; 0x10],
    pub keys: HashMap<Keycode, usize>,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            key_status: [false; 0x10],
            keys: [
                (Keycode::Num1, 0x1),
                (Keycode::Num2, 0x2),
                (Keycode::Num3, 0x3),
                (Keycode::Num4, 0xC),
                (Keycode::Q, 0x4),
                (Keycode::W, 0x5),
                (Keycode::E, 0x6),
                (Keycode::R, 0xD),
                (Keycode::A, 0x7),
                (Keycode::S, 0x8),
                (Keycode::D, 0x9),
                (Keycode::F, 0xE),
                (Keycode::Z, 0xA),
                (Keycode::X, 0x0),
                (Keycode::C, 0xB),
                (Keycode::V, 0xF),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn compute_keycode(&self, keycode: Keycode) -> Option<usize> {
        match self.keys.get(&keycode) {
            Some(chip8_key) => Some(*chip8_key),
            None => None,
        }
    }

    pub fn get_status(&mut self, pos: usize) -> bool {
        self.key_status[pos]
    }

    pub fn being_pressed(&self) -> Option<u8> {
        for key in 0x0..0x10 {
            if self.key_status[key] {
                return Some(key as u8);
            }
        }
        None
    }

    pub fn press(&mut self, key: usize) {
        self.key_status[key] = true;
    }

    pub fn release(&mut self, key: usize) {
        self.key_status[key] = false;
    }
}
