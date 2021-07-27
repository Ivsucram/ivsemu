use std::collections::HashMap;

pub struct OpCodes {
    pub opcode: u16,
    pub n1: u8,
    pub n2: u8,
    pub n3: u8,
    pub n4: u8,
    pub x: usize,
    pub y: usize,
    pub n: u8,
    pub nn: u8,
    pub nnn: usize,
    pub str_to_hex_helper: HashMap<char, Option<u8>>, // Helper
}

impl OpCodes {
    pub fn new(opcode: u16) -> OpCodes {
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0xF00) >> 8;
        let n3 = (opcode & 0xF0) >> 4;
        let n4 = opcode & 0xF;
        let nn = opcode & 0xFF;
        let nnn = opcode & 0xFFF;

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
            nnn: nnn as usize,

            str_to_hex_helper: [
                ('0', Some(0x0)),
                ('1', Some(0x1)),
                ('2', Some(0x2)),
                ('3', Some(0x3)),
                ('4', Some(0x4)),
                ('5', Some(0x5)),
                ('6', Some(0x6)),
                ('7', Some(0x7)),
                ('8', Some(0x8)),
                ('9', Some(0x9)),
                ('A', Some(0xA)),
                ('B', Some(0xB)),
                ('C', Some(0xC)),
                ('D', Some(0xD)),
                ('E', Some(0xE)),
                ('F', Some(0xF)),
                ('?', None),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}
