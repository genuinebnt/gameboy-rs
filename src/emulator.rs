use crate::cartridge::Cartridge;
use crate::cpu::Cpu;

struct Emulator {
    cpu: Cpu,
    cartridge: Cartridge,
}
