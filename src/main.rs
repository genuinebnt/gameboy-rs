mod cpu;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = cpu::Cpu::new();
    let size = cpu.load_rom("../test_roms/cpu_instr.gb", 0)?;
    Ok(())
}
