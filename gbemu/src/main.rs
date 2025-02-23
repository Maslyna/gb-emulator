#![allow(clippy::needless_return)]
extern crate lib_gbemu;
extern crate sdl2;

mod gbscreen;
mod utils;

use utils::ToColor;

use lib_gbemu::cartridge::rom::Rom;
use lib_gbemu::cpu::Cpu;
use lib_gbemu::debug::GBDebug;
use lib_gbemu::memory::Bus;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::error::Error;
use std::sync::{Arc, Condvar, Mutex};

use lib_gbemu::gpu::{X_RES, Y_RES};

const SCALE: u32 = 3;

const DGB_SERIAL: bool = false;
const DBG_H_TYLES: u32 = 16;
const DBG_W_TYLES: u32 = 32;
const DBG_SCREEN_WIDTH: u32 = (DBG_H_TYLES * 8 * SCALE) + (DBG_H_TYLES * SCALE);
const DBG_SCREEN_HEIGHT: u32 = (DBG_W_TYLES * 8 * SCALE) + (DBG_W_TYLES * SCALE);
const DBG_H_ENUM: std::ops::Range<u32> = 0..16;
const DBG_W_ENUM: std::ops::Range<u32> = 0..24;

struct Emulator(Cpu, Bus);

fn window_display(canvas: &mut Canvas<Window>, bus: &Bus) {
    use sdl2::rect::Rect;
    let buffer = &bus.ppu.video_buffer;

    for line in 0..Y_RES {
        for x in 0..X_RES {
            let index = (x + (line * X_RES)) as usize;
            let rect = Rect::new((x * SCALE) as i32, (line * SCALE) as i32, SCALE, SCALE);
            let color = buffer[index].to_color();

            canvas.set_draw_color(color);
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}

fn debug_ui_update(canvas: &mut Canvas<Window>, bus: &Bus) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let address = 0x8000;

    for tile_y in DBG_W_ENUM {
        for tile_x in DBG_H_ENUM {
            gbscreen::display_tile(
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

    canvas.present();
}

fn ui_init() -> (Canvas<Window>, Canvas<Window>, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("gbemu", X_RES * SCALE, Y_RES * SCALE)
        .position_centered()
        .build()
        .unwrap();
    let debug_window = video_subsystem
        .window("DEBUG", DBG_SCREEN_WIDTH, DBG_SCREEN_HEIGHT)
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

    Ok(Emulator(cpu, bus))
}

fn emu_step(cpu: &mut Cpu, bus: &mut Bus, debug: &mut GBDebug) -> bool {
    if bus.emu.paused {
        return true;
    }

    cpu.step(bus);

    if DGB_SERIAL {
        debug.update(bus);
        debug.print();
    }

    true
}

fn main() {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let Emulator(cpu, bus) = create_emu(path).unwrap();
    let bus = Arc::new((Mutex::new(bus), Condvar::new()));

    let (mut canvas, mut debug_canvas, mut event_pump) = ui_init();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    debug_canvas.set_draw_color(Color::RGB(17, 17, 17));
    debug_canvas.clear();
    debug_canvas.present();

    let bus_clone = Arc::clone(&bus);

    std::thread::spawn(move || {
        let mut cpu = cpu;
        let (bus_lock, condvar) = &*bus_clone;
        let mut debug = GBDebug::new();
        loop {
            let mut bus = bus_lock.lock().unwrap();

            if !emu_step(&mut cpu, &mut bus, &mut debug) {
                return;
            }

            condvar.notify_all();
        }
    });

    let mut prev_frame: u32 = 0;
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
        let (bus_lock, condvar) = &*bus;
        let bus = bus_lock.lock().unwrap();
        let bus = condvar.wait(bus).unwrap();

        if prev_frame != bus.ppu.current_frame {
            window_display(&mut canvas, &bus);
            debug_ui_update(&mut debug_canvas, &bus);
        }

        prev_frame = bus.ppu.current_frame;
    }
}
