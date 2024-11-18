use std::error::Error;

use debug::GBDebug;
use crate::cpu::Cpu;
use crate::io::timer::Timer;
use crate::memory::Bus;

#[derive(Debug)]
pub struct Emu {
    pub die: bool,
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
}

impl Emu {
    pub const fn new() -> Self {
        Self {
            die: false,
            paused: false,
            running: false,
            ticks: 0,
        }
    }

    pub fn cycle(&mut self, timer: &mut Timer, cycles: i32) {
        let time = cycles * 4;

        for _ in 0..time {
            self.ticks += 1;
            timer.tick();
        }
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_emu(mut cpu: Cpu, mut bus: Bus, mut emu: Emu) -> Result<(), Box<dyn Error>> {
    let mut debug = GBDebug::new();

    emu.running = true;
    while emu.running {
        if emu.paused {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }

        let cycles = cpu.step(&mut emu, &mut bus);
        debug.update(&mut bus);
        debug.print();
        emu.cycle(&mut bus.timer, cycles);
        bus.step();
    }

    Ok(())
}
