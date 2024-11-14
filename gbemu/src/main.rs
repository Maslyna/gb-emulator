#![allow(clippy::needless_return)]
extern crate lib_gbemu;
extern crate sdl2;

use lib_gbemu::cartridge::rom::Rom;
use lib_gbemu::cpu::Cpu;
use lib_gbemu::emu::Emu;
use lib_gbemu::memory::Bus;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use std::env;
use std::error::Error;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Emulator(Cpu, Bus, Emu);

fn ui_init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("gbemu", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn run_emu(mut cpu: Cpu, mut bus: Bus, mut emu: Emu) -> Result<(), Box<dyn Error>> {
    emu.running = true;
    while emu.running {
        if emu.paused {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let cycles = cpu.step(&mut emu, &mut bus);
        emu.cycle(cycles);

        emu.ticks += 1;
    }

    Ok(())
}

fn create_emu(path: String) -> Result<Emulator, Box<dyn Error>> {
    let (rom, header) = Rom::load(path)?;
    println!("{header}");
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let cpu = Cpu::with_pc(0x100);
    let bus = Bus::new(rom);
    let emu = Emu::new();

    Ok(Emulator(cpu, bus, emu))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let Emulator(cpu, bus, emu) = create_emu(path)?;

    std::thread::spawn(|| {
        run_emu(cpu, bus, emu).unwrap();
    });

    ui_init();

    Ok(())
}
