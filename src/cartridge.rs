use std::fs;

pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge { rom: Vec::new() }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<usize, std::io::Error> {
        let contents = fs::read(path)?;
        contents.into_iter().for_each(|value| {
            self.rom.push(value);
        });

        Ok(self.rom.len())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_rom_test() {}
}
