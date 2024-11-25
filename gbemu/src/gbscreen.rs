use lib_gbemu::memory::Bus;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::SCALE;

const DEBUG: bool = false;

pub fn display_tile(
    bus: &Bus,
    canvas: &mut Canvas<Window>,
    address: u16,
    tile_num: u16,
    x: u32,
    y: u32,
) {
    const TILE_COLORS: [Color; 4] = [
        Color::WHITE,
        Color::RGB(175, 175, 175), // Grey1
        Color::RGB(85, 85, 85),    // Grey2
        Color::BLACK,
    ];

    for tile_y in (0..16).step_by(2) {
        let byte1: u8 = bus.read(address + (tile_num * 16) + tile_y);
        let byte2: u8 = bus.read(address + (tile_num * 16) + tile_y + 1);

        if DEBUG {
            println!(
                "Tile {} Byte1: {:08b}, Byte2: {:08b}",
                tile_num, byte1, byte2
            );
        }
        
        for bit in (0..7).rev() {
            let hi = !!(byte1 & (1 << bit)) << 1;
            let lo = !!(byte2 & (1 << bit));

            let color = TILE_COLORS[if (hi | lo) >= 4 { 0 } else { hi | lo } as usize];

            // draw rectangle
            let rect_x: i32 = (x + (7 - bit) * SCALE) as i32;
            let rect_y = (y + tile_y as u32 / 2 * SCALE) as i32;
            let rect_w = SCALE;
            let rect_h = SCALE;

            let rect = sdl2::rect::Rect::new(rect_x, rect_y, rect_w, rect_h);
            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }
}
