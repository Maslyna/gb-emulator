use sdl2::pixels::Color;

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for lib_gbemu::gpu::Color {
    fn to_color(self) -> Color {
        let a = (self >> 24) as u8;
        let r = (self >> 16) as u8;
        let g = (self >> 8) as u8;
        let b = (self) as u8;

        Color::RGBA(r, g, b, a)
    }
}
