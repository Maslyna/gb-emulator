const TILE_COLORS: [u32; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];

pub enum LcdMode {
    HBlank,
    VBlank,
    Oam,
    Xfer,
}

pub enum StatInterruptSource {
    HBlank = (1 << 3),
    VBlank = (1 << 4),
    Oam = (1 << 5),
    Lyc = (1 << 6),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lcd {
    lcdc: u8,
    lcds: u8,
    scroll_y: u8,
    scrool_x: u8,
    ly: u8,
    ly_compare: u8,
    dma: u8,
    bg_palette: u8,
    obj_palette: u8,
    win_x: u8,
    win_y: u8,

    bg_colors: [u32; 4],
    sp1_colors: [u32; 4],
    sp2_colors: [u32; 4],
}

enum Pallete {
    BgColors,
    Sp1,
    Sp2,
}

impl Lcd {
    pub const fn new() -> Self {
        Self {
            lcdc: 0x91,
            lcds: 0,
            scrool_x: 0,
            scroll_y: 0,
            ly: 0,
            ly_compare: 0,
            dma: 0,
            bg_palette: 0xFC,
            obj_palette: 0xFF,
            win_x: 0,
            win_y: 0,
            bg_colors: TILE_COLORS,
            sp1_colors: TILE_COLORS,
            sp2_colors: TILE_COLORS,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let offset = address.wrapping_sub(0xFF40);

        unsafe {
            let ptr = self as *const Lcd as *const u8;
            *ptr.add(offset as usize)
        }
    }

    // dma_start shoud be called outside of write function
    pub fn write(&mut self, address: u16, value: u8) {
        let offset = address.wrapping_sub(0xFF40);

        unsafe {
            let ptr = self as *mut Lcd as *mut u8;
            *ptr.add(offset as usize) = value;
        }

        match address {
            0xFF47 => self.set_pallete(value, Pallete::BgColors),
            0xFF48 => self.set_pallete(value & 0b11111100, Pallete::Sp1),
            0xFF49 => self.set_pallete(value & 0b11111100, Pallete::Sp2),
            _ => (),
        }
    }

    pub fn lyc(&self) -> bool {
        bit!(self.lcds, 2)
    }

    pub fn set_lyc(&mut self, value: bool) {
        set_bit!(self.lcds, 2, value);
    }

    pub fn is_bgw(&self) -> bool {
        bit!(self.lcdc, 0)
    }

    pub fn is_obj(&self) -> bool {
        bit!(self.lcdc, 1)
    }

    pub fn is_window_enabled(&self) -> bool {
        bit!(self.lcdc, 5)
    }

    pub fn is_lcd_enabled(&self) -> bool {
        bit!(self.lcdc, 7)
    }

    pub fn obj_height(&self) -> u8 {
        if bit!(self.lcdc, 2) {
            16
        } else {
            8
        }
    }

    pub fn bg_map_area(&self) -> u16 {
        if bit!(self.lcdc, 3) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bg_data_area(&self) -> u16 {
        if bit!(self.lcdc, 4) {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn win_map_area(&self) -> u16 {
        if bit!(self.lcdc, 6) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn get_mode(&self) -> LcdMode {
        match self.lcds & 0b0000_0011 {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::Oam,
            3 => LcdMode::Xfer,
            _ => unreachable!(),
        }
    }

    pub fn get_stat_interrupt(&self, src: StatInterruptSource) -> u8 {
        self.lcds & (src as u8)
    }

    pub fn set_mode(&mut self, mode: LcdMode) {
        self.lcds = (self.lcds & 0b1111_1100) | (mode as u8);
    }

    fn set_pallete(&mut self, data: u8, palette: Pallete) {
        match palette {
            Pallete::Sp1 => self.bg_colors = self.sp1_colors,
            Pallete::Sp2 => self.bg_colors = self.sp2_colors,
            Pallete::BgColors => (),
        };

        self.bg_colors
            .iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                *color = TILE_COLORS[((data >> (i * 2)) & 0b0000_0011) as usize];
            });
    }
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new()
    }
}
