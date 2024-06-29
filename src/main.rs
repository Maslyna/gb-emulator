#[allow(clippy::needless_return)]
mod cartridge;

use cartridge::load_cart;
use std::env;
use std::error::Error;

#[allow(clippy::needless_return)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    let path: String = args.nth(1).expect("<PATH> - path to the file");

    let rom = match load_cart(path) {
        Ok(r) => r,
        Err(err) => return Err(Box::new(err)),
    };
    print!("{}", rom);

    return Ok(());
}
