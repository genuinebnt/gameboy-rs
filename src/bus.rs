use std::fs;

#[derive(Debug)]
pub struct Bus {
    pub ram: [u8; 0xFFFF],
}

impl Bus {
    pub fn new() -> Self {
        Bus { ram: [0; 0xFFFF] }
    }
    pub fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.ram[addr] = value;
    }

    pub fn load_rom(&mut self, path: &str, addr: usize) {
        fs::read(path)
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, value)| {
                self.write(addr + i, *value);
            });
    }
}
