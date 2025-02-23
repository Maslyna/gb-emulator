use super::lcd::Lcd;
use super::LcdMode;
use super::StatInterruptSource;
use crate::emu::Emu;
use crate::memory::interrupts::Interrupt;
use crate::memory::Bus;

use std::collections::VecDeque;

use super::{Color, X_RES, Y_RES};

const LINES_PER_FRAME: u32 = 154;
const TICKS_PER_LINE: u32 = 456;
const FRAME_BUFFER_SIZE: usize = (X_RES * Y_RES) as usize;
const DEBUG: bool = false;

#[derive(Debug, Default)]
enum FetchState {
    #[default]
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}

#[derive(Debug, Default)]
struct PixelContext {
    fetch_state: FetchState,
    pixel_info: VecDeque<Color>,
    line_x: u8,
    pushed_x: u8,
    fetch_x: u8,
    bgw_fetch_data: [u8; 3],
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
    tile: u8,
    flags: u8,
}

#[allow(dead_code)]
impl Oam {
    pub const fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            tile: 0,
            flags: 0,
        }
    }

    const F_CGB_PN_MASK: u8 = 0b00000111; // Bits 0-2
    const F_CGB_VRAM_BANK_MASK: u8 = 0b00001000; // Bit 3
    const F_PN_MASK: u8 = 0b00010000; // Bit 4
    const F_X_FLIP_MASK: u8 = 0b00100000; // Bit 5
    const F_Y_FLIP_MASK: u8 = 0b01000000; // Bit 6
    const F_BGP_MASK: u8 = 0b10000000; // Bit 7

    // Getter and setter for f_cgb_pn (bits 0-2)
    pub fn f_cgb_pn(&self) -> u8 {
        self.flags & Self::F_CGB_PN_MASK
    }

    pub fn set_f_cgb_pn(&mut self, value: u8) {
        self.flags = (self.flags & !Self::F_CGB_PN_MASK) | (value & Self::F_CGB_PN_MASK);
    }

    // Getter and setter for f_cgb_vram_bank (bit 3)
    pub fn f_cgb_vram_bank(&self) -> bool {
        self.flags & Self::F_CGB_VRAM_BANK_MASK != 0
    }

    pub fn set_f_cgb_vram_bank(&mut self, value: bool) {
        if value {
            self.flags |= Self::F_CGB_VRAM_BANK_MASK;
        } else {
            self.flags &= !Self::F_CGB_VRAM_BANK_MASK;
        }
    }

    // Getter and setter for f_pn (bit 4)
    pub fn f_pn(&self) -> bool {
        self.flags & Self::F_PN_MASK != 0
    }

    pub fn set_f_pn(&mut self, value: bool) {
        if value {
            self.flags |= Self::F_PN_MASK;
        } else {
            self.flags &= !Self::F_PN_MASK;
        }
    }

    // Getter and setter for f_x_flip (bit 5)
    pub fn f_x_flip(&self) -> bool {
        self.flags & Self::F_X_FLIP_MASK != 0
    }

    pub fn set_f_x_flip(&mut self, value: bool) {
        if value {
            self.flags |= Self::F_X_FLIP_MASK;
        } else {
            self.flags &= !Self::F_X_FLIP_MASK;
        }
    }

    // Getter and setter for f_y_flip (bit 6)
    pub fn f_y_flip(&self) -> bool {
        self.flags & Self::F_Y_FLIP_MASK != 0
    }

    pub fn set_f_y_flip(&mut self, value: bool) {
        if value {
            self.flags |= Self::F_Y_FLIP_MASK;
        } else {
            self.flags &= !Self::F_Y_FLIP_MASK;
        }
    }

    // Getter and setter for f_bgp (bit 7)
    pub fn f_bgp(&self) -> bool {
        self.flags & Self::F_BGP_MASK != 0
    }

    pub fn set_f_bgp(&mut self, value: bool) {
        if value {
            self.flags |= Self::F_BGP_MASK;
        } else {
            self.flags &= !Self::F_BGP_MASK;
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Ppu {
    pub lcd: Lcd,
    pub interrupts: u8,

    oam_ram: [Oam; 0x40],
    vram: [u8; 0x2000],

    line_sprites: VecDeque<Oam>,
    fetched_entry_count: u8,
    fetched_entries: [Oam; 3],
    pub window_line: u8,

    pfc: PixelContext,

    pub current_frame: u32,
    line_ticks: u32,
    pub video_buffer: [u32; FRAME_BUFFER_SIZE],

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

            fetched_entry_count: 0,
            line_sprites: VecDeque::new(),
            fetched_entries: [Oam::new(); 3],
            window_line: 0,

            vram: [0; 0x2000],
            current_frame: 0,
            line_ticks: 0,
            video_buffer: [0; FRAME_BUFFER_SIZE],
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

    fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupts |= interrupt as u8;
    }

    pub fn increment_ly(&mut self) {
        if self.lcd.window_visible()
            && self.lcd.ly >= self.lcd.win_y
            && (self.lcd.ly as u32) < self.lcd.win_y as u32 + Y_RES
        {
            self.window_line = self.window_line.wrapping_add(1);
        }

        self.lcd.ly += 1;

        if self.lcd.ly == self.lcd.ly_compare {
            self.lcd.set_lyc(1);

            if self.lcd.get_stat_interrupt(StatInterruptSource::Lyc) != 0 {
                self.set_interrupt(Interrupt::LcdStat);
            }
        } else {
            self.lcd.set_lyc(0);
        }
    }

    fn pixel_fifo_push(&mut self, value: Color) {
        self.pfc.pixel_info.push_back(value);
    }

    fn pixel_fifo_pop(&mut self) -> Color {
        self.pfc
            .pixel_info
            .pop_front()
            .expect("PIXEL FIFO IS EMPTY!")
    }

    fn pipeline_push_pixel(&mut self) {
        if self.pfc.pixel_info.len() > 8 {
            let pixel_data = self.pixel_fifo_pop();

            if self.pfc.line_x >= (self.lcd.scroll_x % 8) {
                let index = self.pfc.pushed_x as usize + self.lcd.ly as usize * X_RES as usize;
                self.video_buffer[index] = pixel_data;

                self.pfc.pushed_x += 1;
            }

            self.pfc.line_x += 1;
        }
    }

    fn pipeline_fifo_add(&mut self) -> bool {
        if self.pfc.pixel_info.len() > 8 {
            return false;
        }

        let x = (self.pfc.fetch_x.wrapping_sub(8 - (self.lcd.scroll_x % 8))) as i32;

        for bit in (0..8).rev() {
            let lo: u8 = ((self.pfc.bgw_fetch_data[1] & (1 << bit)) != 0) as u8;
            let hi: u8 = (((self.pfc.bgw_fetch_data[2] & (1 << bit)) != 0) as u8) << 1;

            let mut color = self.lcd.bg_colors[(hi | lo) as usize];

            if self.lcd.is_bgw_enabled() == 0 {
                color = self.lcd.bg_colors[0];
            }

            if self.lcd.is_obj_enabled() != 0 {
                color = if let Some(new_color) = self.fetch_sprite_pixels(hi | lo) {
                    new_color
                } else {
                    color
                }
            }

            if x >= 0 {
                self.pixel_fifo_push(color);
                self.pfc.fifo_x += 1;
            }
        }

        true
    }

    fn pipeline_fifo_reset(&mut self) {
        self.pfc.pixel_info.clear();
    }

    fn fetch_sprite_pixels(&mut self, bg_color: u8) -> Option<Color> {
        let mut result: Option<Color> = None;
        for i in 0..(self.fetched_entry_count as usize) {
            let sp_x = self.fetched_entries[i].x - 8 + (self.lcd.scroll_x % 8);

            if sp_x + 8 < self.pfc.fifo_x {
                // Passed pixel point already
                continue;
            }

            let offset = (self.pfc.fifo_x - sp_x) as i32;

            if !(0..=7).contains(&offset) {
                // out of bounds
                continue;
            }

            let mut bit = 7 - offset;

            if self.fetched_entries[i].f_x_flip() {
                bit = offset;
            }
            let hi = (((self.pfc.fetch_entry_data[i * 2] & (1 << bit)) != 0) as u8) << 1;
            let lo = ((self.pfc.fetch_entry_data[(i * 2) + 1] & (1 << bit)) != 0) as u8;

            let bg_priority = self.fetched_entries[i].f_bgp();

            if (hi | lo) == 0 {
                continue; // color is transparent
            }

            if !bg_priority || bg_color == 0 {
                result = Some(if self.fetched_entries[i].f_pn() {
                    self.lcd.sp2_colors[(hi | lo) as usize]
                } else {
                    self.lcd.sp1_colors[(hi | lo) as usize]
                });

                if (hi | lo) != 0 {
                    break;
                }
            }
        }

        result
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn ppu_tick(&mut self) {
        if DEBUG {
            let debug_ppu_output = format!(
                "PPU: FRAME {} FCOUNT {} LINET {} ",
                self.ppu.current_frame, self.ppu.frame_count, self.ppu.line_ticks
            );
            let debug_lcd_output: String = format!(
                "LCD: LCDC {} LCDS {} LY {} DMA {}",
                self.ppu.lcd.lcdc, self.ppu.lcd.lcds, self.ppu.lcd.ly, self.ppu.lcd.dma
            );
            let debug_pixelcontext_output: String = format!(
                "PFC: STATE {:?} PI_LEN {}",
                self.ppu.pfc.fetch_state,
                self.ppu.pfc.pixel_info.len()
            );

            let debug_output =
                format!("{debug_ppu_output}\n{debug_lcd_output}\n{debug_pixelcontext_output}\n");
            print!("{debug_output}");
            crate::common::debug_write(&debug_output);
        }

        self.ppu.line_ticks += 1;

        match self.ppu.lcd.get_lcds_mode() {
            LcdMode::Oam => self.mode_oam(),
            LcdMode::Xfer => self.mode_xfer(),
            LcdMode::VBlank => self.mode_vblank(),
            LcdMode::HBlank => self.mode_hblank(),
        };
    }

    fn pipeline_fetch(&mut self) {
        match self.ppu.pfc.fetch_state {
            FetchState::Tile => {
                self.ppu.fetched_entry_count = 0;

                if self.ppu.lcd.is_bgw_enabled() != 0 {
                    let address = self.ppu.lcd.bg_map_area()
                        + (self.ppu.pfc.map_x / 8) as u16
                        + ((self.ppu.pfc.map_y / 8) as u16 * 32);
                    self.ppu.pfc.bgw_fetch_data[0] = self.read(address);

                    if self.ppu.lcd.bgw_data_area() == 0x8800 {
                        self.ppu.pfc.bgw_fetch_data[0] =
                            self.ppu.pfc.bgw_fetch_data[0].wrapping_add(128);
                    }

                    self.pipline_load_window_tile()
                }

                if self.ppu.lcd.is_obj_enabled() != 0 && self.ppu.line_sprites.is_empty() {
                    self.pipeline_load_sprite_title();
                }

                self.ppu.pfc.fetch_state = FetchState::Data0;
                self.ppu.pfc.fetch_x = self.ppu.pfc.fetch_x.wrapping_add(8);
            }
            FetchState::Data0 => {
                // TODO: I have +1, but why?
                let address = self.ppu.lcd.bgw_data_area()
                    + (self.ppu.pfc.bgw_fetch_data[0] as u16 * 16)
                    + self.ppu.pfc.tile_y as u16
                    + 1;
                self.ppu.pfc.bgw_fetch_data[1] = self.read(address);

                self.pipeline_load_sprite_data(0);

                self.ppu.pfc.fetch_state = FetchState::Data1;
            }
            FetchState::Data1 => {
                let address = self.ppu.lcd.bgw_data_area()
                    + (self.ppu.pfc.bgw_fetch_data[0] as u16 * 16)
                    + (self.ppu.pfc.tile_y + 1) as u16;
                self.ppu.pfc.bgw_fetch_data[2] = self.read(address);

                self.pipeline_load_sprite_data(1);

                self.ppu.pfc.fetch_state = FetchState::Idle;
            }
            FetchState::Idle => {
                self.ppu.pfc.fetch_state = FetchState::Push;
            }
            FetchState::Push => {
                if self.ppu.pipeline_fifo_add() {
                    self.ppu.pfc.fetch_state = FetchState::Tile;
                }
            }
        };
    }

    fn pipeline_process(&mut self) {
        let pfc = &mut self.ppu.pfc;
        let lcd = &mut self.ppu.lcd;

        pfc.map_y = lcd.ly.wrapping_add(lcd.scroll_y);
        pfc.map_x = pfc.fetch_x.wrapping_add(lcd.scroll_x);
        pfc.tile_y = (lcd.ly.wrapping_add(lcd.scroll_y) % 8).wrapping_mul(2);

        if (self.ppu.line_ticks & 1) == 0 {
            self.pipeline_fetch();
        }

        self.ppu.pipeline_push_pixel();
    }

    fn pipline_load_window_tile(&mut self) {
        if !self.ppu.lcd.window_visible() {
            return;
        }

        let window_y = self.ppu.lcd.win_y;
        let window_x = self.ppu.lcd.win_x;
        let comp = self.ppu.pfc.fetch_x + 7;

        if comp >= window_x
            && (comp as u32) < (window_x as u32 + Y_RES + 14)
            && self.ppu.lcd.ly >= window_x
            && (self.ppu.lcd.ly as u32) < window_y as u32 + X_RES
        {
            let w_tile_y = self.ppu.window_line / 8;

            self.ppu.pfc.bgw_fetch_data[0] = self.read(
                ((self.ppu.lcd.win_map_area() as i32
                    + (self.ppu.pfc.fetch_x as i32 + 7 - window_x as i32) / 8)
                    + (w_tile_y as i32 * 32)) as u16,
            );

            if self.ppu.lcd.bgw_data_area() == 0x8800 {
                self.ppu.pfc.bgw_fetch_data[0] += 128;
            }
        }
    }

    fn pipeline_load_sprite_title(&mut self) {
        for elem in self.ppu.line_sprites.iter() {
            let sprite_x = (elem.x - 8) + (self.ppu.lcd.scroll_x % 8);

            if (sprite_x >= self.ppu.pfc.fetch_x) && (sprite_x < (self.ppu.pfc.fetch_x + 8))
                || ((sprite_x + 8) >= self.ppu.pfc.fetch_x)
                    && ((sprite_x + 8) < (self.ppu.pfc.fetch_x + 8))
            {
                let index = self.ppu.fetched_entry_count as usize;
                self.ppu.fetched_entry_count += 1;
                self.ppu.fetched_entries[index] = *elem;
            }

            if self.ppu.fetched_entry_count >= 3 {
                break;
            }
        }
    }

    fn pipeline_load_sprite_data(&mut self, offset: u8) {
        let current_y = self.ppu.lcd.ly;
        let sprite_heigth = self.ppu.lcd.obj_height();

        for i in 0..self.ppu.fetched_entry_count as usize {
            let mut tile_y = ((current_y + 16) - self.ppu.fetched_entries[i].y) * 2;

            if self.ppu.fetched_entries[i].f_y_flip() {
                // flipped Y
                tile_y = (sprite_heigth * 2) - 2 - tile_y;
            }

            let mut tile_index = self.ppu.fetched_entries[i].tile;

            if sprite_heigth == 16 {
                tile_index &= !1;
            }
            let address = 0x8000 + (tile_index as u16 * 16) + tile_y as u16 + offset as u16;
            self.ppu.pfc.fetch_entry_data[(i * 2) + offset as usize] = self.read(address);
        }
    }

    fn mode_oam(&mut self) {
        if self.ppu.line_ticks >= 80 {
            self.ppu.lcd.set_lcds_mode(LcdMode::Xfer);

            self.ppu.pfc.fetch_state = FetchState::Tile;
            self.ppu.pfc.line_x = 0;
            self.ppu.pfc.fetch_x = 0;
            self.ppu.pfc.pushed_x = 0;
            self.ppu.pfc.fifo_x = 0;
        }

        if self.ppu.line_ticks == 1 {
            self.ppu.line_sprites.clear();

            self.load_line_sprites();
        }
    }

    fn mode_xfer(&mut self) {
        self.pipeline_process();

        if self.ppu.pfc.pushed_x >= X_RES as u8 {
            self.ppu.pipeline_fifo_reset();

            self.ppu.lcd.set_lcds_mode(LcdMode::HBlank);

            if self.ppu.lcd.get_stat_interrupt(StatInterruptSource::HBlank) != 0 {
                self.ppu.set_interrupt(Interrupt::LcdStat);
            }
        }
    }

    fn mode_hblank(&mut self) {
        if self.ppu.line_ticks >= TICKS_PER_LINE {
            self.ppu.increment_ly();

            if self.ppu.lcd.ly >= Y_RES as u8 {
                self.ppu.lcd.set_lcds_mode(LcdMode::VBlank);

                self.ppu.set_interrupt(Interrupt::VBlank);

                if self.ppu.lcd.get_stat_interrupt(StatInterruptSource::VBlank) != 0 {
                    self.ppu.set_interrupt(Interrupt::LcdStat);
                }

                self.ppu.current_frame += 1;

                // TODO: into Screen trait
                let end = Emu::get_ticks();
                let frame_time = end.wrapping_sub(self.ppu.prev_frame_time);

                if frame_time < self.ppu.target_frame_time {
                    Emu::delay(self.ppu.target_frame_time - frame_time);
                }

                if end - self.ppu.start_time >= 1000 {
                    // let fps = self.ppu.frame_count;
                    self.ppu.start_time = end;
                    self.ppu.frame_count = 0;

                    // println!("FPS: {fps}");
                }

                self.ppu.frame_count += 1;
                self.ppu.prev_frame_time = Emu::get_ticks();
            } else {
                self.ppu.lcd.set_lcds_mode(LcdMode::Oam);
            }

            self.ppu.line_ticks = 0;
        }
    }

    fn mode_vblank(&mut self) {
        if self.ppu.line_ticks >= TICKS_PER_LINE {
            self.ppu.increment_ly();

            if self.ppu.lcd.ly >= LINES_PER_FRAME as u8 {
                self.ppu.lcd.set_lcds_mode(LcdMode::Oam);
                self.ppu.lcd.ly = 0;
                self.ppu.window_line = 0;
            }

            self.ppu.line_ticks = 0;
        }
    }

    fn load_line_sprites(&mut self) {
        let current_y = self.ppu.lcd.ly;
        let sprite_h = self.ppu.lcd.obj_height();

        for oam in self.ppu.oam_ram {
            if oam.x == 0 {
                continue;
            }

            if self.ppu.line_sprites.len() >= 10 {
                break;
            }

            if oam.y <= current_y + 16 && oam.y + sprite_h > current_y + 16 {
                self.ppu.line_sprites.push_back(oam);
            }
        }
    }
}

impl PixelContext {
    pub fn new() -> Self {
        Self {
            fetch_state: FetchState::Tile,
            pixel_info: VecDeque::new(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
        }
    }
}
