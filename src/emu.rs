use crate::{bus::Bus, cpu::Cpu};

#[derive(Debug)]
pub struct Emu {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
}


impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: false,
            ticks: 0,
        }
    }

    pub fn run(&mut self, cpu: &mut Cpu, bus: &mut Bus) -> Result<(), EmuError> {
        self.running = true;
        while self.running {
            if self.paused {
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            if !cpu.step(self, bus) {
                self.running = false;
                return Err(EmuError::CpuErr("CPU execution failed!".to_string()));
            }

            self.ticks += 1;
        }
        return Ok(());
    }

    pub fn emu_cycles(&mut self, _cycles: i32) {}
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