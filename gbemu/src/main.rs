#[macro_use]
extern crate lib_gbemu;
extern crate sdl2;

mod gbscreen;
mod utils;

use lib_gbemu::{
    cartridge::rom::Rom,
    cpu::Cpu,
    debug::GsSerial,
    gpu::{GbWindow, X_RES, Y_RES},
    io::input::Gamepad,
    memory::Bus,
};

use gbscreen::{DebugMode, DebugWindow, MainWindow};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
};

use std::env;

const SCALE: i32 = 3;
const DGB_SERIAL: bool = false;
const DBG_SCREEN_WIDTH: i32 = (16 * 8 * SCALE) + (16 * SCALE);
const DBG_SCREEN_HEIGHT: i32 = (32 * 8 * SCALE) + (32 * SCALE);

struct Emulator<'a>(Cpu, Bus<'a>);

fn on_key(gamepad: &mut Gamepad, bus: &mut Bus, keycode: Keycode, down: bool) {
    let state = gamepad.get_state_mut();
    match keycode {
        Keycode::Right => state.a = down,
        Keycode::Left => state.b = down,
        Keycode::Down => state.start = down,
        Keycode::Tab => state.select = down,
        Keycode::Up => state.up = down,
        Keycode::Return => state.down = down,
        Keycode::Z => state.left = down,
        Keycode::X => state.right = down,
        _ => {}
    };
    bus.gamepad.set_state(*state);
}

fn ui_init() -> (MainWindow, DebugWindow, sdl2::EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("gbemu", (X_RES * SCALE) as u32, (Y_RES * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();
    let debug_window = video_subsystem
        .window("DEBUG", DBG_SCREEN_WIDTH as u32, DBG_SCREEN_HEIGHT as u32)
        .position(
            window.position().0 + window.size().0 as i32,
            window.position().1,
        )
        .build()
        .unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (
        MainWindow::new(window.into_canvas().build().unwrap()),
        DebugWindow(debug_window.into_canvas().build().unwrap()),
        event_pump,
    )
}

fn create_emu(path: String, screen: &mut dyn GbWindow) -> Result<Emulator, &'static str> {
    let (rom, header) = Rom::load(path)?;
    println!("{header}");

    let cpu = Cpu::new();
    let bus = Bus::new(rom, screen);

    Ok(Emulator(cpu, bus))
}

fn emu_step(cpu: &mut Cpu, bus: &mut Bus, debug: &mut GsSerial) -> bool {
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

    lib_gbemu::common::init_logger();
    
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 8)
        .name("SDL Thread".to_string())
        .spawn(move || {
            let (mut main_window, mut debug_window, mut event_pump) = ui_init();
            main_window.set_draw_color(Color::BLACK);
            main_window.clear();
            main_window.present();

            debug_window.set_draw_color(Color::RGB(17, 17, 17));
            debug_window.clear();
            debug_window.present();

            let mut emulator_window = DebugMode {
                main_window,
                debug_window,
                is_updated: false,
            };

            let Emulator(mut cpu, mut bus) =
                create_emu(path, make_mut_ref!(&mut emulator_window)).unwrap();
            let mut serial = GsSerial::new();

            let mut gamepad = Gamepad::new();

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
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => on_key(&mut gamepad, &mut bus, keycode, true),
                        Event::KeyUp {
                            keycode: Some(keycode),
                            ..
                        } => on_key(&mut gamepad, &mut bus, keycode, false),
                        _ => {}
                    }
                }
                bus.gamepad.set_state(gamepad.state);
                if !emu_step(&mut cpu, &mut bus, &mut serial) {
                    return;
                };
                if emulator_window.is_updated {
                    emulator_window.debug_window.update(&bus);
                    emulator_window.debug_window.present();
                    emulator_window.is_updated = false;
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
