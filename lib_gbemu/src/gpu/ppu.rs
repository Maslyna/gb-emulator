use super::lcd::Lcd;
use super::LcdMode;
use super::StatInterruptSource;
use crate::emu::Emu;
use crate::memory::interrupts::Interrupt;
use crate::memory::Bus;

use std::collections::LinkedList;
use std::sync::Arc;

use super::{X_RES, Y_RES};

const LINES_PER_FRAME: u32 = 154;
const TICKS_PER_LINE: u32 = 456;
const FRAME_BUFFER_SIZE: usize = (X_RES * Y_RES) as usize;

#[derive(Debug)]
enum FetchState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}

#[derive(Debug)]
struct FifoEntry {
    next: Option<Arc<FifoEntry>>,
    value: u32,
}

#[derive(Debug)]
struct PixelContext {
    fetch_state: FetchState,
    pixel_info: LinkedList<FifoEntry>,
    line_x: u8,
    pushed_x: u8,
    fetch_x: u8,
    background_fetch_data: [u8; 3],
    fetch_entry_data: [u8; 6],
    map_y: u8,
    map_x: u8,
    tile_y: u8,
    fifo_x: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Oam {
    y: u8,
    x: u8,
    title: u8,
    flags: u8,
}

impl Oam {
    pub const fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            title: 0,
            flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct Ppu {
    pub lcd: Lcd,
    pub interrupts: u8,
    oam_ram: [Oam; 0x40],
    vram: [u8; 0x2000],

    pfc: PixelContext,

    pub current_frame: u32,
    line_ticks: u32,
    pub video_buffer: Box<[u32; FRAME_BUFFER_SIZE]>,

    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_time: u64,
    pub frame_count: u64,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            lcd: Lcd::new(),
            interrupts: 0,
            oam_ram: [Oam::new(); 0x40],
            vram: [0; 0x2000],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: Box::new([0; FRAME_BUFFER_SIZE]),
            target_frame_time: 1000 / 60, // 60 FPS
            prev_frame_time: 0,
            start_time: 0,
            frame_count: 0,
            pfc: PixelContext::new(),
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
        let index = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        let ptr = self.oam_ram.as_mut_ptr() as *mut u8;
        unsafe {
            *ptr.add(index) = value;
        }
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let index = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        let ptr = self.oam_ram.as_ptr() as *const u8;
        unsafe { *ptr.add(index) }
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        self.vram[address.wrapping_sub(0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[address.wrapping_sub(0x8000) as usize]
    }

    pub fn increment_ly(&mut self) {
        self.lcd.ly += 1;

        if self.lcd.ly == self.lcd.ly_compare {
            self.lcd.set_lyc(true);

            if self.lcd.get_stat_interrupt(StatInterruptSource::Lyc) != 0 {
                self.set_interrupt(Interrupt::LcdStat);
            }
        } else {
            self.lcd.set_lyc(false);
        }
    }

    fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupts |= interrupt as u8;
    }

    fn push_pixel_fifo(&mut self, value: u32) {
        self.pfc
            .pixel_info
            .push_back(FifoEntry { next: None, value });
    }

    fn pop_pixel_fifo(&mut self) -> u32 {
        if let Some(entry) = self.pfc.pixel_info.pop_back() {
            return entry.value;
        }
        panic!("PIXEL FIFO IS EMPTY!");
    }

    fn push_pipeline_pixel(&mut self) {
        if self.pfc.pixel_info.len() > 8 {
            let pixel_data = self.pop_pixel_fifo();

            if self.pfc.line_x >= (self.lcd.scroll_x % 8) {
                self.video_buffer
                    [(self.pfc.pushed_x as u32 + (self.lcd.ly as u32 * X_RES)) as usize] =
                    pixel_data;

                self.pfc.pushed_x += 1;
            }

            self.pfc.line_x += 1;
        }
    }

    fn add_pipeline_fifo(&mut self) -> bool {
        if self.pfc.pixel_info.len() > 8 {
            return false;
        }

        let x: i8 = (self.pfc.fetch_x.wrapping_sub(8 - (self.lcd.scroll_x % 8))) as i8;
        for bit in 0..8 {
            let bit = 7 - bit;
            let lo: u8 = (self.pfc.background_fetch_data[1] & (1 << bit) != 0) as u8;
            let hi: u8 = ((self.pfc.background_fetch_data[2] & (1 << bit)) << 1 != 0) as u8;

            let color = self.lcd.bg_colors[(hi | lo) as usize];

            if x >= 0 {
                self.push_pixel_fifo(color);
                self.pfc.fifo_x += 1;
            }
        }

        true
    }

    fn reset_pipeline_fifo(&mut self) {
        self.pfc.pixel_info.clear();
    }
}

impl Bus {
    pub fn ppu_tick(&mut self) {
        self.ppu.line_ticks += 1;

        match self.ppu.lcd.get_mode() {
            LcdMode::HBlank => self.mode_hblank(),
            LcdMode::VBlank => self.mode_vblank(),
            LcdMode::Oam => self.mode_oam(),
            LcdMode::Xfer => self.mode_xfer(),
        }
    }

    fn fetch_ppu_pipeline(&mut self) {
        match self.ppu.pfc.fetch_state {
            FetchState::Tile => {
                if self.ppu.lcd.is_bgw_enabled() {
                    let address = self.ppu.lcd.bg_map_area()
                        + (self.ppu.pfc.map_x as u16 / 8)
                        + ((self.ppu.pfc.map_y as u16 / 8) * 32);
                    self.ppu.pfc.background_fetch_data[0] = self.read(address);

                    if self.ppu.lcd.bwg_data_area() == 0x8800 {
                        self.ppu.pfc.background_fetch_data[0] =
                            self.ppu.pfc.background_fetch_data[0].wrapping_add(128);
                    }
                }

                self.ppu.pfc.fetch_state = FetchState::Data0;
                self.ppu.pfc.fetch_x = self.ppu.pfc.fetch_x.wrapping_add(8);
            }
            FetchState::Data0 => {
                let address = self.ppu.lcd.bwg_data_area()
                    + (self.ppu.pfc.background_fetch_data[0] as u16 * 16)
                    + self.ppu.pfc.tile_y as u16;
                self.ppu.pfc.background_fetch_data[1] = self.read(address);

                self.ppu.pfc.fetch_state = FetchState::Data1;
            }
            FetchState::Data1 => {
                let address = self.ppu.lcd.bwg_data_area()
                    + (self.ppu.pfc.background_fetch_data[0] as u16 * 16)
                    + self.ppu.pfc.tile_y as u16
                    + 1;
                self.ppu.pfc.background_fetch_data[2] = self.read(address);

                self.ppu.pfc.fetch_state = FetchState::Idle;
            }
            FetchState::Idle => {
                self.ppu.pfc.fetch_state = FetchState::Push;
            }
            FetchState::Push => {
                if self.ppu.add_pipeline_fifo() {
                    self.ppu.pfc.fetch_state = FetchState::Tile;
                }
            }
        };
    }

    fn process_pipeline(&mut self) {
        self.ppu.pfc.map_y = self.ppu.lcd.ly.wrapping_add(self.ppu.lcd.scroll_y);
        self.ppu.pfc.map_x = self.ppu.pfc.fetch_x.wrapping_add(self.ppu.lcd.scroll_x);
        self.ppu.pfc.tile_y = ((self.ppu.lcd.ly + self.ppu.lcd.scroll_y) % 8).wrapping_mul(2);

        if (self.ppu.line_ticks & 1) == 0 {
            self.fetch_ppu_pipeline();
        }

        self.ppu.push_pipeline_pixel();
    }

    fn mode_oam(&mut self) {
        if self.ppu.line_ticks >= 80 {
            self.ppu.lcd.set_mode(LcdMode::Xfer);

            self.ppu.pfc.fetch_state = FetchState::Tile;
            self.ppu.pfc.line_x = 0;
            self.ppu.pfc.fetch_x = 0;
            self.ppu.pfc.pushed_x = 0;
            self.ppu.pfc.fifo_x = 0;
        }
    }

    fn mode_xfer(&mut self) {
        self.process_pipeline();

        if self.ppu.pfc.pushed_x as u32 >= X_RES {
            self.ppu.reset_pipeline_fifo();
            self.ppu.lcd.set_mode(LcdMode::HBlank);

            if self.ppu.lcd.get_stat_interrupt(StatInterruptSource::HBlank) != 0 {
                self.ppu.set_interrupt(Interrupt::LcdStat);
            }
        }
    }

    fn mode_hblank(&mut self) {
        if self.ppu.line_ticks >= TICKS_PER_LINE {
            self.ppu.increment_ly();

            if self.ppu.lcd.ly as u32 >= Y_RES {
                self.ppu.lcd.set_mode(LcdMode::VBlank);
                self.ppu.set_interrupt(Interrupt::VBlank);

                if self.ppu.lcd.get_stat_interrupt(StatInterruptSource::VBlank) != 0 {
                    self.ppu.set_interrupt(Interrupt::LcdStat);
                }

                // TODO: into Screen trait
                let end = Emu::get_ticks();
                let frame_time = end.wrapping_sub(self.ppu.prev_frame_time);

                if frame_time < self.ppu.target_frame_time {
                    Emu::delay(self.ppu.target_frame_time - frame_time);
                }

                if end - self.ppu.start_time >= 1000 {
                    let fps = self.ppu.frame_count;
                    self.ppu.start_time = end;
                    self.ppu.frame_count = 0;

                    println!("FPS: {fps}");
                }

                self.ppu.current_frame += 1;
                self.ppu.prev_frame_time = Emu::get_ticks();
            } else {
                self.ppu.lcd.set_mode(LcdMode::Oam);
            }

            self.ppu.line_ticks = 0;
        }
    }

    fn mode_vblank(&mut self) {
        if self.ppu.line_ticks >= TICKS_PER_LINE {
            self.ppu.lcd.ly += 1;

            if self.ppu.lcd.ly as u32 >= LINES_PER_FRAME {
                self.ppu.lcd.ly = 0;
                self.ppu.lcd.set_mode(LcdMode::Oam);
            }

            self.ppu.line_ticks = 0;
        }
    }
}

impl PixelContext {
    pub fn new() -> Self {
        Self {
            fetch_state: FetchState::Tile,
            pixel_info: LinkedList::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            background_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
