pub mod ppu;
pub mod lcd;

pub type Color = u32;

const DEFAULT_COLORS: [Color; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];
pub const X_RES: i32 = 160;
pub const Y_RES: i32 = 144;


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
