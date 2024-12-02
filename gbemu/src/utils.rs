use sdl2::pixels::Color;

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for u32 {
    fn to_color(self) -> Color {
        let a = ((self >> 24) & 0xFF) as u8;
        let r = ((self >> 16) & 0xFF) as u8;
        let g = ((self >> 8) & 0xFF) as u8;
        let b = (self & 0xFF) as u8;

        Color::RGBA(r, g, b, a)
    }
}