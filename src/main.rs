#![allow(clippy::needless_return)]

#[macro_use]
mod macros;
mod cartridge;
mod cpu;
mod bus;
mod emu;

use bus::Bus;
use cpu::Cpu;
use emu::Emu;
use cartridge::rom::Rom;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let rom = match Rom::load(path) {
        Ok(r) => r,
        Err(err) => return Err(Box::new(err)),
    };
    println!("{}", rom);

    let mut cpu = Cpu::with_pc(0x100);
    let mut bus = Bus::new(rom);
    let mut emu = Emu::new();
    
    return match emu.run(&mut cpu, &mut bus) {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err))
    };
}
