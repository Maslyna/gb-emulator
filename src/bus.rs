// 0x0000 - 0x3FFF : ROM Bank 0
// 0x4000 - 0x7FFF : ROM Bank 1 - Switchable
// 0x8000 - 0x97FF : CHR RAM
// 0x9800 - 0x9BFF : BG Map 1
// 0x9C00 - 0x9FFF : BG Map 2
// 0xA000 - 0xBFFF : Cartridge RAM
// 0xC000 - 0xCFFF : RAM Bank 0
// 0xD000 - 0xDFFF : RAM Bank 1-7 - switchable - Color only
// 0xE000 - 0xFDFF : Reserved - Echo RAM
// 0xFE00 - 0xFE9F : Object Attribute Memory
// 0xFEA0 - 0xFEFF : Reserved - Unusable
// 0xFF00 - 0xFF7F : I/O Registers
// 0xFF80 - 0xFFFE : Zero Page

use crate::cartridge::rom::Rom;

pub struct Bus {
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self { rom }
    }

    pub fn read(&self, address: u16) -> u8 {
        return self.rom.read(address);
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo: u8 = self.read(address);
        let hi: u8 = self.read(address);

        return combine_bytes!(lo, hi);
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x8000 {
            self.rom.write(address, value);
        }

        todo!("NOT IMPLEMENTED WRITING INTO ROM");
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write(address, (value & 0xFF) as u8);
    }
}
