pub struct ProgramCounter(pub usize);

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter { 0: 0x200 }
    }

    pub fn increment(&mut self) {
        self.0 += 2;
    }

    pub fn decrement(&mut self) {
        self.0 -= 2;
    }
}
