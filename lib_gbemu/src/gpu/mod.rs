pub mod ppu;
pub mod lcd;

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
