#![allow(clippy::needless_return)]

mod macros;
mod cartridge;
mod cpu;
mod bus;

use cartridge::rom::Rom;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let path: String = args.nth(1).expect("<PATH> - path to the file");
    println!("PATH: {}", path);
    
    let rom = match Rom::load(path) {
        Ok(r) => r,
        Err(err) => return Err(Box::new(err)),
    };
    print!("{}", rom);

    return Ok(());
}
