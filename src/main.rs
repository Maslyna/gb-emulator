#![allow(clippy::needless_return)]

#[macro_use]
mod macros;
mod bus;
mod cartridge;
mod cpu;
mod emu;
mod ram;

use bus::Bus;
use cartridge::rom::Rom;
use cpu::Cpu;
use emu::Emu;

use std::env;
use std::error::Error;

type Emulator = (Cpu, Bus, Emu);

fn run_emu(mut cpu: Cpu, mut bus: Bus, mut emu: Emu) -> Result<(), Box<dyn Error>> {
    emu.running = true;
    while emu.running {
        if emu.paused {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        if cpu.halted {
            panic!("CPU EXEC FAILED");
        }

        cpu.fetch_instruction(&bus);
        let cycles = cpu.fetch_data(&bus);
        emu.cycle(cycles);
        let cycles = cpu.execute(&mut bus, &mut emu);
        emu.cycle(cycles);

        emu.ticks += 1;
    }

    Ok(())
}

fn create_emu(path: String) -> Result<Emulator, Box<dyn Error>> {
    let (rom, header) = Rom::load(path)?;
    println!("{header}");

    let cpu = Cpu::with_pc(0x100);
    let bus = Bus::new(rom);
    let emu = Emu::new();

    Ok((cpu, bus, emu))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let (cpu, bus, emu) = create_emu(path)?;
    run_emu(cpu, bus, emu)?;

    Ok(())
}
