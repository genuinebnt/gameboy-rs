struct Registers {
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

struct Cpu {
    opcode: u32,
    registers: Registers,
    ram: [u8; 0xFFFF],
}
