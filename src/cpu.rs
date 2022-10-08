use crate::bus::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        (flags.z as u8) << 7 | (flags.n as u8) << 6 | (flags.h as u8) << 5 | (flags.c as u8) << 4
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: (value & 0b10000000) >> 7 == 1,
            n: (value & 0b01000000) >> 6 == 1,
            h: (value & 0b00100000) >> 5 == 1,
            c: (value & 0b00010000) >> 4 == 1,
        }
    }
}

pub struct Cpu {
    opcode: u32,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    bus: Bus,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            opcode: 0,
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
            sp: 0,
            pc: 0,
            bus: Bus::new(),
        }
    }

    pub fn tick(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_flags() {
        let flags = Flags {
            z: true,
            n: true,
            h: false,
            c: true,
        };

        assert_eq!(0b11010000, u8::from(flags));
        assert_eq!(Flags::from(0b11010000), flags);
    }

    fn load_rom_test() {
        let cpu = Cpu::new();
    }
}
