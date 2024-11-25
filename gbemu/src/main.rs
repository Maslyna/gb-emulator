#![allow(clippy::needless_return)]
extern crate lib_gbemu;
extern crate sdl2;

mod gbscreen;

use lib_gbemu::cartridge::rom::Rom;
use lib_gbemu::cpu::Cpu;
use lib_gbemu::debug::GBDebug;
use lib_gbemu::emu::Emu;
use lib_gbemu::memory::Bus;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::env;
use std::error::Error;
use std::sync::{Arc, Condvar, Mutex};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const SCALE: u32 = 2;

struct Emulator(Cpu, Bus, Emu);

fn debug_ui_update(canvas: &mut Canvas<Window>, bus: &Bus) {
    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_num = 0;

    let address = 0x8000;

    // 384 tiles -> 24 * 16
    for tile_y in 0..24 {
        for tile_x in 0..16 {
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

    let Emulator(cpu, bus, emu) = create_emu(path).unwrap();
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
        let mut emu = emu;
        let mut debug = GBDebug::new();
        loop {
            let mut bus = bus_lock.lock().unwrap();

            if !emu_step(&mut cpu, &mut bus, &mut emu, &mut debug) {
                return;
            }

            condvar.notify_all();
        }
    });

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

        debug_ui_update(&mut debug_canvas, &bus);
    }
}
