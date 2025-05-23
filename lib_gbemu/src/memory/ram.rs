#[derive(Debug)]
pub struct Ram {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

impl Ram {
    pub const fn new() -> Self {
        Self {
            wram: [0u8; 0x2000],
            hram: [0u8; 0x80],
        }
    }

    pub fn wram_read(&self, mut address: u16) -> u8 {
        address -= 0xC000;

        self.wram[address as usize]
    }

    pub fn wram_write(&mut self, mut address: u16, value: u8) {
        address -= 0xC000;

        self.wram[address as usize] = value;
    }

    pub fn hram_read(&self, mut address: u16) -> u8 {
        address -= 0xFF80;

        self.hram[address as usize]
    }

    pub fn hram_write(&mut self, mut address: u16, value: u8) {
        address -= 0xFF80;

        self.hram[address as usize] = value;
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
