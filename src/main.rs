mod bus;
mod cpu;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = cpu::Cpu::new();
    cpu.registers.f.z = true;
    println!("{:x}", u8::from(cpu.registers.f));

    Ok(())
}
