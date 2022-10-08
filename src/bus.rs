pub struct Bus {
    memory: [u8; 0xFFFF],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: [0; 0xFFFF],
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
