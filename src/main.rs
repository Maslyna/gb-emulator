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
use emu::{Emu, EmuError};

use std::env;
use std::error::Error;

use std::cell::RefCell;
use std::rc::Rc;

fn setup(cpu: Rc<RefCell<Cpu>>, bus: Rc<RefCell<Bus>>, emu: Rc<RefCell<Emu>>) {
    cpu.borrow_mut().bus = Some(bus.clone());
    cpu.borrow_mut().emu = Some(emu.clone());
    bus.borrow_mut().cpu = Some(cpu.clone());
}

fn run_emu(cpu: Rc<RefCell<Cpu>>, emu: Rc<RefCell<Emu>>) -> Result<(), Box<dyn Error>> {
    emu.borrow_mut().running = true;
    while emu.borrow().running {
        if emu.borrow().paused {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        if !cpu.borrow_mut().step() {
            emu.borrow_mut().running = false;
            return Err(Box::new(EmuError::CpuErr(
                "CPU execution failed!".to_string(),
            )));
        }

        emu.borrow_mut().ticks += 1;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args();
    let path: String = args.last().expect("<PATH> - path to the file");
    println!("PATH: {}", path);

    let rom = Rom::load(path)?;
    println!("{}", rom);

    let cpu = Rc::new(RefCell::new(Cpu::with_pc(0x100)));
    let bus = Rc::new(RefCell::new(Bus::new(rom)));
    let emu = Rc::new(RefCell::new(Emu::new()));

    setup(cpu.clone(), bus.clone(), emu.clone());

    emu.borrow_mut().running = true;
    run_emu(cpu, emu)?;

    Ok(())
}
