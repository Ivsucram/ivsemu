pub struct RAM {
    ram: [u8; 4 * 1024],
    font_address: usize,
    rom_address: usize,
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            ram: [0x00; 4 * 1024],
            font_address: 0x50 as usize,
            rom_address: 0x200 as usize,
        }
    }

    pub fn init_fonts(&mut self) {
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..fonts.len() {
            self.ram[i + self.font_address] = fonts[i];
        }
    }

    pub fn get_font_address(&self) -> usize {
        self.font_address
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        assert!(
            rom.len() <= self.ram.len() - 0x200,
            "ROM is bigger than Chip-8 RAM"
        );
        for i in 0..rom.len() {
            self.ram[self.rom_address + i] = rom[i];
        }
    }

    pub fn read8(&self, addr: usize) -> u8 {
        assert!(
            addr <= self.ram.len(),
            "addr = {}, self.0.len() = {}",
            addr,
            self.ram.len()
        );
        self.ram[addr]
    }

    pub fn read16(&self, addr: usize) -> u16 {
        assert!(
            addr < self.ram.len(),
            "addr = {}, self.0.len() = {}",
            addr,
            self.ram.len()
        );
        (self.ram[addr] as u16) << 8 | self.ram[addr + 1] as u16
    }

    pub fn write8(&mut self, addr: usize, value: u8) {
        assert!(
            addr < self.ram.len(),
            "addr = {}, self.0.len() = {}",
            addr,
            self.ram.len()
        );
        self.ram[addr] = value;
    }
}
