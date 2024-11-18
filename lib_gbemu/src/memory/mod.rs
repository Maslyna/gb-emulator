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

pub mod interrupts;
pub mod ram;
use self::interrupts::*;

use self::ram::Ram;
use crate::cartridge::rom::Rom;
use crate::io::timer::Timer;

#[derive(Debug)]
pub struct Bus {
    pub interrupts: InterruptState,

    rom: Rom,
    ram: Ram,
    pub timer: Timer,
    serial_data: [u8; 2],
}

impl Bus {
    pub const fn new(rom: Rom) -> Self {
        Self {
            rom,
            ram: Ram::new(),
            interrupts: InterruptState::new(),
            timer: Timer::new(),
            serial_data: [0; 2],
        }
    }

    pub fn step(&mut self) {
        self.interrupts.flags |= self.timer.interrupts;
        self.timer.interrupts = 0;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // ROM DATA
            ..0x8000 => self.rom.read(address),
            // Char/Map DATA
            0x8000..0xA000 => {
                eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                0
            }
            // Cartridge RAM
            0xA000..0xC000 => self.rom.read(address),
            // WRAM
            0xC000..0xE000 => self.ram.wram_read(address),
            // ECO RAM
            0xE000..0xFE00 => {
                eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                0
            }
            // OAM
            0xFE00..0xFEA0 => {
                eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                0
            }
            // Reserved unusable
            0xFEA0..0xFF00 => {
                eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                0
            }
            // IO Registers
            0xFF00..0xFF80 => {
                // println!("IO READ: {:04X}", address);
                match address {
                    0xFF01 => self.serial_data[0],
                    0xFF02 => self.serial_data[1],
                    0xFF04..=0xFF07 => self.timer.read(address),
                    0xFF0F => self.interrupts.flags,
                    _ => {
                        eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                        0
                    }
                }
            }
            // CPU ENABLED REGISTERS
            interrupts::INTERRUPT_ENABLE_ADDRESS => self.interrupts.enabled,
            _ => self.ram.hram_read(address),
        }
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo: u8 = self.read(address);
        let hi: u8 = self.read(address);

        bytes_to_word!(lo, hi)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        debug!("Write in bus: {address:04X}, value: {value:02X}");
        if address == 0xFF02 && value == 0x81 {
            println!("BREAKPOINT");
        }
        match address {
            // ROM DATA
            ..0x8000 => self.rom.write(address, value),
            // Char/Map DATA
            0x8000..0xA000 => eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // EXT-RAM
            0xA000..0xC000 => self.rom.write(address, value),
            // WRAM
            0xC000..0xE000 => self.ram.wram_write(address, value),
            // Reserved echo RAM
            0xE000..0xFE00 => eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // OAM
            0xFE00..0xFEA0 => eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // Reserved unusable
            0xFEA0..0xFF00 => eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // IO Registers 0xFF00..0xFF80
            0xFF00..0xFF80 => {
                // println!("IO WRITE: {:04X}, {:02X}", address, value);
                match address {
                    0xFF01 => self.serial_data[0] = value,
                    0xFF02 => self.serial_data[1] = value,
                    0xFF04..=0xFF07 => self.timer.write(address, value),
                    0xFF0F => self.interrupts.flags = value,
                    _ => eprintln!("UNSUPPORTED BUS WRITE {:04X} VALUE {:04X}", address, value),
                }
            }
            // CPU SET ENABLE REGISTER
            0xFFFF => self.interrupts.enabled = value,
            _ => self.ram.hram_write(address, value),
        }
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write(address, (value & 0xFF) as u8);
    }
}
