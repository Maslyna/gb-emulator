use crate::cartridge::rom::Rom;
use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct Emu {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    pub cpu: Cpu,
    pub bus: Bus,
}


impl Emu {
    pub fn new(rom: Rom) -> Self {
        Self {
            paused: false,
            running: false,
            ticks: 0,
            cpu: Cpu::with_pc(0x100),
            bus: Bus::new(rom),
        }
    }

    pub fn run(&mut self) -> Result<(), EmuError> {
        self.running = true;
        while self.running {
            if self.paused {
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            if !self.cpu.step(&mut self.bus) {
                self.running = false;
                return Err(EmuError::CpuErr("CPU execution failed!".to_string()));
            }

            self.ticks += 1;
        }
        return Ok(());
    }
}


#[derive(Debug)]
pub enum EmuError {
    CpuErr(String),
}

impl std::fmt::Display for EmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmuError::CpuErr(msg) => write!(f, "CPU encountered an error: {}", msg),
        }
    }
}

impl std::error::Error for EmuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}