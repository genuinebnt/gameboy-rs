pub fn disassemble(ram: &[u8], pc: usize) -> usize {
    let mut op_len = 1;

    let opcode = ram[pc];

    match opcode {
        0x00 => {
            println!("NOP")
        }
        0x01 => {
            println!("LD BC,0x{:X}{:X}", ram[pc + 2], ram[pc + 1]);
            op_len = 2;
        }
        0x02 => {
            println!("LD (BC),A")
        }
        0x03 => {
            println!("INC BC");
        }
        0x04 => {
            println!("INC B");
        }
        0x05 => {
            println!("DEC B");
        }
        0x06 => {
            println!("LD B,0x{:X}", ram[pc + 1]);
            op_len = 2;
        }

        0x07 => {
            println!("RLCA")
        }

        0x08 => {
            println!("LD (0x{:X}{:X}), SP", ram[pc + 2], ram[pc + 1])
        }

        _ => {}
    }

    op_len
}
