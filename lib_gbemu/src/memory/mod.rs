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

mod dma;

pub mod interrupts;
pub mod ram;

use self::dma::Dma;
use self::interrupts::*;
use self::ram::Ram;
use crate::cartridge::rom::Rom;
use crate::emu::Emu;
use crate::gpu::ppu::Ppu;
use crate::io::gamepad::Gamepad;
use crate::io::timer::Timer;

#[derive(Debug)]
pub struct Bus {
    pub interrupts: InterruptState,

    rom: Rom,
    ram: Ram,
    pub ppu: Ppu,
    dma: Dma,
    pub emu: Emu,
    pub timer: Timer,

    pub gamepad: Gamepad,

    serial_data: [u8; 2],
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Self {
            ppu: Ppu::new(),
            rom,
            dma: Dma::new(),
            ram: Ram::new(),
            emu: Emu::new(),
            interrupts: InterruptState::new(),
            timer: Timer::new(),

            gamepad: Gamepad::new(),

            serial_data: [0; 2],
        }
    }

    pub fn cycle(&mut self, cycles: i32) {
        for _ in 0..cycles {
            for _ in 0..4 {
                self.timer.ticks = self.timer.ticks.wrapping_add(1);
                self.timer.tick();
                self.interrupts.flags |= self.timer.interrupts;
                self.timer.interrupts = 0;

                self.ppu_tick();
            }

            self.dma_tick();
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            interrupts::INTERRUPT_FLAGS_ADDRESS => self.interrupts.flags,
            // CPU ENABLED REGISTERS
            interrupts::INTERRUPT_ENABLE_ADDRESS => self.interrupts.enabled,
            // ROM DATA
            0..0x8000 => self.rom.read(address),
            // Char/Map DATA
            0x8000..0xA000 => self.ppu.vram_read(address),
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
                if self.dma.is_transfering() {
                    return 0xFF;
                }
                self.ppu.oam_read(address)
            }
            // Reserved unusable
            0xFEA0..0xFF00 => 0,
            // IO Registers
            0xFF00..0xFF80 => match address {
                0xFF00 => self.gamepad.calculate_output(),
                0xFF01 => self.serial_data[0],
                0xFF02 => self.serial_data[1],
                0xFF04..=0xFF07 => self.timer.read(address),

                0xFF40..=0xFF4B => self.ppu.lcd.read(address),
                _ => {
                    eprintln!("UNSUPPORTED BUS READ {:04X}", address);
                    0
                }
            },
            _ => self.ram.hram_read(address),
        }
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo: u8 = self.read(address);
        let hi: u8 = self.read(address);

        bytes_to_word!(lo, hi)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // CPU SET ENABLE REGISTER
            interrupts::INTERRUPT_ENABLE_ADDRESS => self.interrupts.enabled = value,
            interrupts::INTERRUPT_FLAGS_ADDRESS => self.interrupts.flags = value,
            // ROM DATA
            ..0x8000 => self.rom.write(address, value),
            // Char/Map DATA
            0x8000..0xA000 => self.ppu.vram_write(address, value),
            // EXT-RAM
            0xA000..0xC000 => self.rom.write(address, value),
            // WRAM
            0xC000..0xE000 => self.ram.wram_write(address, value),
            // Reserved echo RAM
            0xE000..0xFE00 => eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // OAM
            0xFE00..0xFEA0 => {
                if self.dma.is_transfering() {
                    return;
                }
                self.ppu.oam_write(address, value);
            }
            // Reserved unusable
            0xFEA0..0xFF00 => (), //eprintln!("UNSUPPORTED BUS WRITE {:04X}", address),
            // IO Registers
            0xFF00..0xFF80 => match address {
                0xFF00 => self.gamepad.set_selector(value),
                0xFF01 => self.serial_data[0] = value,
                0xFF02 => self.serial_data[1] = value,
                0xFF04..=0xFF07 => self.timer.write(address, value),
                0xFF40..=0xFF4B => {
                    if (address - 0xFF40) == 0xFF46 {
                        self.dma.start(value);
                    }
                    self.ppu.lcd.write(address, value);
                }
                _ => (), // eprintln!("UNSUPPORTED BUS WRITE {:04X} VALUE {:04X}", address, value),
            },
            _ => self.ram.hram_write(address, value),
        }
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write(address, (value & 0xFF) as u8);
    }
}
