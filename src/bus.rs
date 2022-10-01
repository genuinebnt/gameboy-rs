#[derive(Debug)]
pub struct Bus {
    ram: [u8; 0xFFFF],
}

impl Bus {
    pub fn new() -> Self {
        Bus { ram: [0; 0xFFFF] }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }
}
