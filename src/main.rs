mod cartridge;

use cartridge::load_cart;

fn main() {
    let file = load_cart("[path]");
    match file {
        Ok(_fil) => println!("SUCCESS"),
        Err(e) => println!("{}", e)
    }
}