use std::fs;
use std::io::Error;
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: usize,
    pc: usize,
}

pub struct Cpu {
    opcode: u32,
    registers: Registers,
    ram: [u8; 0xFFFF],
    stack: [u8; 0xFF],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            opcode: 0,
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                f: 0,
                h: 0,
                l: 0,
                sp: 0,
                pc: 0,
            },
            ram: [0; 0xFFFF],
            stack: [0; 0xFF],
        }
    }

    pub fn load_rom(&mut self, path: &str, address: usize) -> Result<usize, Error> {
        let contents = fs::read(path)?;
        self.ram[address..address + contents.len()].copy_from_slice(&contents);

        Ok(contents.len())
    }

    pub fn tick(&mut self) {
        
    }
}
