pub mod ppu;
pub mod lcd;

pub type Color = u32;

const COLORS_DEFAULT: [Color; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];
pub const X_RES: u32 = 160;
pub const Y_RES: u32 = 144;


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
