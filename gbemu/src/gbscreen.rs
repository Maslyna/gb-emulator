use lib_gbemu::{
    gpu::{Color as GbColor, GbWindow, X_RES, Y_RES},
    memory::Bus,
};

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc::{channel, Sender},
    Arc, Mutex,
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

fn delay(milis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milis));
}

fn get_ticks() -> u64 {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get current time in millis")
        .as_millis() as u64
}

type SharedCounter = Arc<AtomicUsize>;

struct FrameTask {
    start_y: usize,
    end_y: usize,
    buffer: Arc<[GbColor]>,
    pixel_data: Arc<Mutex<Vec<u8>>>,
    done_counter: SharedCounter,
}

pub struct MainWindow {
    pub canvas: Canvas<Window>,
    pub target_frame_time: u64,
    pub prev_frame_time: u64,
    pub start_time: u64,
    pub frame_count: u64,
    thread_senders: Vec<Sender<FrameTask>>,
}

impl MainWindow {
    pub fn new(canvas: Canvas<Window>) -> Self {
        const NUM_THREADS: usize = 4;

        let mut thread_senders = Vec::new();

        for _ in 0..NUM_THREADS {
            let (tx, rx) = channel::<FrameTask>();
            thread_senders.push(tx);

            std::thread::spawn(move || {
                while let Ok(task) = rx.recv() {
                    let FrameTask {
                        start_y,
                        end_y,
                        buffer,
                        pixel_data,
                        done_counter,
                    } = task;

                    let mut local = vec![0u8; X_RES as usize * (end_y - start_y) * 3];
                    for y in start_y..end_y {
                        for x in 0..X_RES as usize {
                            let index = x + y * X_RES as usize;
                            let color = buffer[index].to_color();
                            let pixel_index = (y - start_y) * X_RES as usize * 3 + x * 3;
                            local[pixel_index] = color.r;
                            local[pixel_index + 1] = color.g;
                            local[pixel_index + 2] = color.b;
                        }
                    }

                    {
                        let mut pd = pixel_data.lock().unwrap();
                        let offset = start_y * X_RES as usize * 3;
                        pd[offset..offset + local.len()].copy_from_slice(&local);
                    }

                    done_counter.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
        Self {
            canvas,
            target_frame_time: 0,
            prev_frame_time: 0,
            frame_count: 0,
            start_time: 0,
            thread_senders,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    #[inline(always)]
    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }
}

impl GbWindow for MainWindow {
    #[inline(always)]
    fn draw_frame(&mut self, buffer: &[GbColor]) {
        let end = get_ticks();
        let frame_time = end - self.prev_frame_time;

        if frame_time < self.target_frame_time {
            std::thread::sleep(std::time::Duration::from_millis(
                self.target_frame_time - frame_time,
            ));
        }

        if end - self.start_time >= 1000 {
            println!("FPS: {}", self.frame_count);
            self.start_time = end;
            self.frame_count = 0;
        }

        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGB24,
                X_RES as u32,
                Y_RES as u32,
            )
            .unwrap();

        let shared_pixel_data = Arc::new(Mutex::new(vec![0u8; (X_RES * Y_RES * 3) as usize]));
        let shared_buffer: Arc<[GbColor]> = Arc::from(buffer.to_vec().into_boxed_slice());
        let done_counter = Arc::new(AtomicUsize::new(0));
        let num_threads = self.thread_senders.len();
        let chunk_height = Y_RES as usize / num_threads;

        for (i, tx) in self.thread_senders.iter().enumerate() {
            let start_y = i * chunk_height;
            let end_y = if i == num_threads - 1 {
                Y_RES as usize
            } else {
                (i + 1) * chunk_height
            };

            tx.send(FrameTask {
                start_y,
                end_y,
                buffer: shared_buffer.clone(),
                pixel_data: shared_pixel_data.clone(),
                done_counter: done_counter.clone(),
            })
            .unwrap();
        }

        while done_counter.load(Ordering::SeqCst) < num_threads {
            std::thread::yield_now();
        }

        let pixel_data = shared_pixel_data.lock().unwrap();
        texture
            .update(None, &pixel_data, (X_RES * 3) as usize)
            .unwrap();
        self.canvas.copy(&texture, None, None).unwrap();
        self.canvas.present();

        self.frame_count += 1;
        self.prev_frame_time = get_ticks();
    }

    #[inline(always)]
    fn present(&mut self) {
        self.canvas.present();
    }
}

pub struct DebugWindow(pub Canvas<Window>);

pub struct DebugMode {
    pub main_window: MainWindow,
    pub debug_window: DebugWindow,
    pub is_updated: bool,
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
