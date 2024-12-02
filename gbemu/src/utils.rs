use sdl2::pixels::Color;

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for u32 {
    fn to_color(self) -> Color {
        let swapped = self.swap_bytes().reverse_bits();
        let a = (swapped >> 24) as u8;
        let r = (swapped >> 16) as u8;
        let g = (swapped >> 8) as u8;
        let b = (swapped) as u8;

        Color::RGBA(r, g, b, a)
    }

    // fn to_color(self) -> Color {
    //     let r = ((self >> 16) & 0xFF) as u8;
    //     let g = ((self >> 8) & 0xFF) as u8;
    //     let b = (self & 0xFF) as u8;
    //     Color::RGB(r, g, b)
    // }
}