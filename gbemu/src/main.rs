#![allow(clippy::needless_return)]
extern crate lib_gbemu;
extern crate sdl2;

use lib_gbemu::cartridge::rom::Rom;
use lib_gbemu::cpu::Cpu;
use lib_gbemu::debug::GBDebug;
use lib_gbemu::emu::Emu;
use lib_gbemu::memory::Bus;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::error::Error;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SCALE: u32 = 2;

struct Emulator(Cpu, Bus, Emu);

fn display_tile(
    bus: &Bus,
    canvas: &mut Canvas<Window>,
    address: u16,
    tile_num: u16,
    x: u32,
    y: u32,
) {
    const TILE_COLORS: [Color; 4] = [
        Color::RGB(255, 255, 255),
        Color::RGB(175, 175, 175),
        Color::RGB(85, 85, 85),
        Color::RGB(0, 0, 0),
    ];

    for tile_y in (0..16).step_by(2) {
        let byte1: u8 = bus.read(address + (tile_num * 16) + tile_y);
        let byte2: u8 = bus.read(address + (tile_num * 16) + tile_y + 1);

        for bit in (0..7).rev() {
            let hi = (((byte1 & (1 << bit)) != 0) as u8) << 1;
            let lo = ((byte2 & (1 << bit)) != 0) as u8;

            let color = TILE_COLORS[(hi | lo) as usize];

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

fn debug_ui_update(canvas: &mut Canvas<Window>, bus: &Bus) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let address = 0x8000;

    // 384 tiles -> 24 * 16
    for tile_y in 0..24 {
        for tile_x in 0..16 {
            display_tile(
                bus,
                canvas,
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

fn ui_create_windows() -> (Canvas<Window>, Canvas<Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("gbemu", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let debug_window = video_subsystem
        .window(
            "DEBUG",
            (16 * 8 * SCALE) + (16 * SCALE),
            (32 * 8 * SCALE) + (64 * SCALE),
        )
        .position(
            window.position().0 + window.size().0 as i32,
            window.position().1,
        )
        .build()
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (
        window.into_canvas().build().unwrap(),
        debug_window.into_canvas().build().unwrap(),
        event_pump,
    )
}

fn create_emu(path: String) -> Result<Emulator, Box<dyn Error>> {
    let (rom, header) = Rom::load(path)?;
    println!("{header}");

    let cpu = Cpu::new();
    let bus = Bus::new(rom);
    let emu = Emu::new();

    Ok(Emulator(cpu, bus, emu))
}

fn emu_step(cpu: &mut Cpu, bus: &mut Bus, emu: &mut Emu, debug: &mut GBDebug) -> bool {
    if emu.paused {
        return true;
    }

    let cycles = cpu.step(emu, bus);
    debug.update(bus);
    debug.print();
    emu.cycle(&mut bus.timer, cycles);
    bus.step();

    true
}

fn main() {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let Emulator(mut cpu, mut bus, mut emu) = create_emu(path).unwrap();
    let mut debug = GBDebug::new();
    let (mut canvas, mut debug_canvas, mut event_pump) = ui_create_windows();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    debug_canvas.set_draw_color(Color::RGB(17, 17, 17));
    debug_canvas.clear();
    debug_canvas.present();

    'gb_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gb_loop,
                _ => {}
            }
        }

        if !emu_step(&mut cpu, &mut bus, &mut emu, &mut debug) {
            return;
        }

        debug_ui_update(&mut debug_canvas, &bus);
    }
}
