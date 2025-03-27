use lib_gbemu::{
    gpu::{Color as GbColor, GbWindow, X_RES, Y_RES},
    memory::Bus,
};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use utils::ToColor;

use crate::SCALE;

const DEBUG: bool = false;

pub const TILE_COLORS: [Color; 4] = [
    Color::RGB(255, 255, 255),
    Color::RGB(175, 175, 175),
    Color::RGB(85, 85, 85),
    Color::RGB(0, 0, 0),
];

pub struct MainWindow {
    pub canvas: Canvas<Window>,
    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_time: u64,
    pub frame_count: u64,
}

pub struct DebugWindow(pub Canvas<Window>);

pub struct DebugMode {
    pub main_window: MainWindow,
    pub debug_window: DebugWindow,
    pub is_updated: bool,
}

fn delay(milis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milis));
}

fn get_ticks() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get current time in millis")
        .as_millis() as u64
}

impl MainWindow {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            canvas,
            target_frame_time: 0,
            prev_frame_time: 0,
            frame_count: 0,
            start_time: 0,
        }
    }
}

impl GbWindow for MainWindow {
    #[inline(always)]
    fn draw_frame(&mut self, buffer: &[GbColor]) {
        let end = get_ticks();
        let frame_time = end - self.prev_frame_time;

        if frame_time < self.target_frame_time {
            delay(self.target_frame_time - frame_time);
            println!("Delay");
        }

        if end - self.start_time >= 1000 {
            let fps = self.frame_count;
            self.start_time = end;
            self.frame_count = 0;
            println!("FPS: {}", fps);
        }

        for line in 0..Y_RES {
            for x in 0..X_RES {
                let index = (x + (line * X_RES)) as usize;
                let rect = Rect::new(x * SCALE, line * SCALE, SCALE as u32, SCALE as u32);
                let color = buffer[index].to_color();

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.frame_count += 1;
        self.prev_frame_time = get_ticks();
    }

    #[inline(always)]
    fn present(&mut self) {
        self.canvas.present();
    }
}

impl MainWindow {
    #[inline(always)]
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    #[inline(always)]
    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }
}

impl DebugWindow {
    #[inline(always)]
    fn display_tile(
        bus: &Bus,
        canvas: &mut Canvas<Window>,
        address: u16,
        tile_num: u16,
        x: i32,
        y: i32,
    ) {
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
                let hi = (((byte1 & (1 << bit)) != 0) as u8) << 1;
                let lo = ((byte2 & (1 << bit)) != 0) as u8;

                let color_id = hi | lo;
                let color = TILE_COLORS[color_id as usize];

                // draw rectangle
                let rect_x: i32 = x + (7 - bit) * SCALE;
                let rect_y = y + tile_y as i32 / 2 * SCALE;
                let rect_w = SCALE;
                let rect_h = SCALE;

                let rect = Rect::new(rect_x, rect_y, rect_w as u32, rect_h as u32);
                canvas.set_draw_color(color);
                canvas.fill_rect(rect).unwrap();
            }
        }
    }

    #[inline(always)]
    pub fn update(&mut self, bus: &Bus) {
        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_num = 0;

        let address = 0x8000;

        for tile_y in 0..24 {
            for tile_x in 0..16 {
                Self::display_tile(
                    bus,
                    &mut self.0,
                    address,
                    tile_num,
                    x_draw + (tile_x * SCALE),
                    y_draw + (tile_y * SCALE),
                );

                x_draw += 8 * SCALE;
                tile_num += 1;
            }

            x_draw = 0;
            y_draw += 8 * SCALE;
        }
    }

    #[inline(always)]
    pub fn present(&mut self) {
        self.0.present();
    }
}

impl DebugWindow {
    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline(always)]
    pub fn set_draw_color(&mut self, color: Color) {
        self.0.set_draw_color(color);
    }
}

impl GbWindow for DebugMode {
    #[inline(always)]
    fn draw_frame(&mut self, buffer: &[GbColor]) {
        self.is_updated = true;
        self.main_window.draw_frame(buffer);
    }

    #[inline(always)]
    fn present(&mut self) {
        self.main_window.present();
    }
}
