use super::{Color, LcdMode, StatInterruptSource};

const PALLETTE_COLORS: [Color; 4] = super::DEFAULT_COLORS;

#[derive(Debug)]
#[repr(C)]
pub struct Lcd {
    pub lcdc: u8,             // FF40
    pub lcds: u8,             // FF41
    pub scroll_y: u8,         // FF42
    pub scroll_x: u8,         // FF43
    pub ly: u8,               // FF44
    pub ly_compare: u8,       // FF45
    pub dma: u8,              // FF46
    pub bg_palette: u8,       // FF47
    pub obj_palette: [u8; 2], // FF48 FF49
    pub win_x: u8,            // FF4A
    pub win_y: u8,            // FF4B

    pub bg_colors: [Color; 4],
    pub sp1_colors: [Color; 4],
    pub sp2_colors: [Color; 4],
}

#[repr(u8)]
enum Pallete {
    BgColors,
    Sp1,
    Sp2,
}

impl Lcd {
    pub fn new() -> Self {
        let mut lcd = Self {
            lcdc: 0x91,
            lcds: 0,
            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            ly_compare: 0,
            dma: 0,
            bg_palette: 0xFC,
            obj_palette: [0xFF; 2],
            win_x: 0,
            win_y: 0,
            bg_colors: PALLETTE_COLORS,
            sp1_colors: PALLETTE_COLORS,
            sp2_colors: PALLETTE_COLORS,
        };
        lcd.set_lcds_mode(LcdMode::Oam);
        lcd
    }

    pub fn read(&self, address: u16) -> u8 {
        let offset = address - 0xFF40;

        unsafe {
            let ptr = self as *const _ as *const u8;
            *ptr.add(offset as usize)
        }
    }

    // dma_start shoud be called outside of write function
    pub fn write(&mut self, address: u16, value: u8) {
        let offset = address.wrapping_sub(0xFF40);

        unsafe {
            let ptr = self as *mut _ as *mut u8;
            *ptr.add(offset as usize) = value;
        }

        match address {
            0xFF47 => self.set_pallete(value, Pallete::BgColors),
            0xFF48 => self.set_pallete(value & 0b1111_1100, Pallete::Sp1),
            0xFF49 => self.set_pallete(value & 0b1111_1100, Pallete::Sp2),
            _ => (),
        }
    }

    pub fn lyc(&self) -> u8 {
        self.lcds & 0b0000_0100
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lcds = (self.lcds & 0b1111_1011) | (value << 2);
    }

    pub fn is_bgw_enabled(&self) -> u8 {
        self.lcdc & 0b0000_0001
    }

    pub fn is_obj_enabled(&self) -> u8 {
        self.lcdc & 0b0000_0010
    }

    pub fn is_window_visible(&self) -> bool {
        self.is_window_enabled() != 0 && self.win_x <= 166 && self.win_y < super::Y_RES as u8
    }

    pub fn is_window_enabled(&self) -> u8 {
        self.lcdc & 0b0010_0000
    }

    pub fn is_lcd_enabled(&self) -> bool {
        bit!(self.lcdc, 7)
    }

    pub fn obj_height(&self) -> u8 {
        if self.lcdc & 0b0000_0100 != 0 {
            16
        } else {
            8
        }
    }

    pub fn bg_map_area(&self) -> u16 {
        if self.lcdc & 0b0000_1000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bgw_data_area(&self) -> u16 {
        if self.lcdc & 0b0001_0000 != 0 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn win_map_area(&self) -> u16 {
        if self.lcdc & 0b0100_0000 != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn get_lcds_mode(&self) -> LcdMode {
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

    pub fn set_lcds_mode(&mut self, mode: LcdMode) {
        self.lcds = (self.lcds & 0b1111_1100) | (mode as u8);
    }

    fn set_pallete(&mut self, data: u8, palette: Pallete) {
        self.bg_colors = match palette {
            Pallete::Sp1 => self.sp1_colors,
            Pallete::Sp2 => self.sp2_colors,
            Pallete::BgColors => self.bg_colors,
        };

        self.bg_colors[0] = PALLETTE_COLORS[(data & 0b11) as usize];
        self.bg_colors[1] = PALLETTE_COLORS[((data >> 2) & 0b11) as usize];
        self.bg_colors[2] = PALLETTE_COLORS[((data >> 4) & 0b11) as usize];
        self.bg_colors[3] = PALLETTE_COLORS[((data >> 6) & 0b11) as usize];
    }
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new()
    }
}
