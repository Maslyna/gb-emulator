use super::{LcdMode, StatInterruptSource};

const PALETTE_COLORS: [u32; 4] = super::COLORS_DEFAULT;

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct Lcd {
    pub lcdc: u8,
    pub lcds: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub ly: u8,
    pub ly_compare: u8,
    pub dma: u8,
    pub bg_palette: u8,
    pub obj_palette: [u8; 2],
    pub win_x: u8,
    pub win_y: u8,

    pub bg_colors: [u32; 4],
    pub sp1_colors: [u32; 4],
    pub sp2_colors: [u32; 4],
}

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
            bg_colors: PALETTE_COLORS,
            sp1_colors: PALETTE_COLORS,
            sp2_colors: PALETTE_COLORS,
        };

        lcd.set_mode(LcdMode::Oam);

        lcd
    }

    pub fn read(&self, address: u16) -> u8 {
        let offset = address.wrapping_sub(0xFF40);

        match offset {
            0x00 => self.lcdc,
            0x01 => self.lcds,
            0x02 => self.scroll_y,
            0x03 => self.scroll_x,
            0x04 => self.ly,
            0x05 => self.ly_compare,
            0x06 => self.dma,
            0x07 => self.bg_palette,
            0x08 => self.obj_palette[0],
            0x09 => self.obj_palette[1],
            0x0A => self.win_y,
            0x0B => self.win_x,
            _ => unreachable!(),
        }
    }

    // dma_start shoud be called outside of write function
    pub fn write(&mut self, address: u16, value: u8) {
        let offset = address.wrapping_sub(0xFF40);

        match offset {
            0x00 => self.lcdc = value,
            0x01 => self.lcds = value,
            0x02 => self.scroll_y = value,
            0x03 => self.scroll_x = value,
            0x04 => self.ly = value,
            0x05 => self.ly_compare = value,
            0x06 => self.dma = value,
            0x07 => self.bg_palette = value,
            0x08 => self.obj_palette[0] = value,
            0x09 => self.obj_palette[1] = value,
            0x0A => self.win_y = value,
            0x0B => self.win_x = value,
            _ => unreachable!(),
        };

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

    pub fn is_bgw_enabled(&self) -> bool {
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

    pub fn bwg_data_area(&self) -> u16 {
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
                *color = PALETTE_COLORS[((data >> (i * 2)) & 0b0000_0011) as usize];
            });
    }
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new()
    }
}
