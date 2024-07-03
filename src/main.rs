#![allow(clippy::needless_return)]

mod macros;
mod cartridge;
mod cpu;
mod bus;
mod emu;

use cartridge::rom::Rom;
use emu::{Emu, EmuError};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let a: Vec<String> = env::args().collect();
    println!("args = {:?}", a);
    let path: String = args.nth(1).expect("<PATH> - path to the file");
    println!("PATH: {}", path);
    
    let rom = match Rom::load(path) {
        Ok(r) => r,
        Err(err) => return Err(Box::new(err)),
    };
    println!("{}", rom);

    let mut emu = Emu::new(rom);
    
    
    return match emu.run() {
        Ok(_) => Ok(()),
        Err(err) => Err(Box::new(err))
    };
}
