use std::convert::From;

use crate::bus::Bus;
use crate::disassembler::disassemble;

enum Instructions {}

// Cpu flags from f register
#[derive(Debug)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}

// convert from u8 to cpu flags. In gameboy only top 4 bits are set rest are defaulted to 0
impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: ((value & 0b10000000) >> 7) != 0,
            n: ((value & 0b01000000) >> 6) != 0,
            h: ((value & 0b00100000) >> 5) != 0,
            c: ((value & 0b00010000) >> 4) != 0,
        }
    }
}

//convert from flags to u8
impl From<Flags> for u8 {
    fn from(registers: Flags) -> Self {
        (registers.z as u8) << 7
            | (registers.n as u8) << 6
            | (registers.h as u8) << 5
            | (registers.c as u8) << 4
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub opcode: u32,
    pub registers: Registers,
    pub sp: u16,
    pub pc: u16,
    pub bus: Bus,
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
                f: Flags {
                    z: false,
                    n: false,
                    h: false,
                    c: false,
                },
                h: 0,
                l: 0,
            },
            sp: 0,
            pc: 0,
            bus: Bus::new(),
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.opcode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        let mut cpu = Cpu::new();
        let flags = Flags {
            z: true,
            n: true,
            h: false,
            c: false,
        };

        cpu.registers.f = flags;

        assert_eq!(0xC0, u8::from(cpu.registers.f));
    }

    #[test]
    fn test_disassembler() {
        let mut bus = Bus::new();
        bus.load_rom("test_roms/06-ld r,r.gb", 0);

        //println!("{:?}", bus.ram);

        let mut pc = 0;

        for _ in 0..1000 {
            pc += disassemble(&bus.ram, pc);
        }
    }
}
