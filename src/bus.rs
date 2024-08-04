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
use crate::ram::Ram;

pub struct Bus {
    rom: Rom,
    ram: Ram,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            rom,
            ram: Ram::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // ROM DATA
            ..0x8000 => self.rom.read(address),
            // Char/Map DATA
            0x8000..0xA000 => todo!("UNSUPPORTED BUS READ {address:04X}"),
            // Cartridge RAM
            0xA000..0xC000 => self.rom.read(address),
            // WRAM
            0xC000..0xE000 => self.ram.wram_read(address),
            // ECO RAM
            0xE000..0xFE00 => 0,
            // OAM
            0xFE00..0xFEA0 => todo!("UNSUPPORTED BUS READ {address:04X}"),
            // Reserved unusable
            0xFEA0..0xFF00 => 0,
            // IO Registers
            0xFF00..0xFF80 => todo!("UNSUPPORTED BUS READ {address:04X}"),
            // CPU ENABLED REGISTERS
            0xFFFF => todo!("UNSUPPORTED BUS READ {address:04X}"),
            _ => self.ram.hram_read(address),
        }
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo: u8 = self.read(address);
        let hi: u8 = self.read(address);

        return combine_bytes!(lo, hi);
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // ROM DATA
            ..0x8000 => self.rom.write(address, value),
            // Char/Map DATA
            0x8000..0xA000 => todo!("UNSUPPORTED BUS WRITE {address:04X}"),
            // EXT-RAM
            0xA000..0xC000 => self.rom.write(address, value),
            // WRAM
            0xC000..0xE000 => self.ram.wram_write(address, value),
            // Reserved echo RAM
            0xE000..0xFE00 => (),
            // OAM
            0xFE00..0xFEA0 => todo!("UNSUPPORTED BUS WRITE {address:04X}"),
            // Reserved unusable
            0xFEA0..0xFF00 => (),
            // IO Registers
            0xFF00..0xFF80 => todo!("UNSUPPORTED BUS WRITE {address:04X}"),
            // CPU SET ENABLE REGISTER
            0xFFFF => todo!("UNSUPPORTED BUS WRITE {address:04X}"),
            _ => self.ram.hram_write(address, value),
        }
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write(address, (value & 0xFF) as u8);
    }
}
