
use crate::memory::interrupts::Interrupt;

#[derive(Debug)]
pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    
    pub interrupts: u8,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
            interrupts: 0
        }
    }

    pub fn tick(&mut self) {
        let prev_divider = self.div;
        self.div = self.div.wrapping_add(1);

        let mut timer_update = false;

        match self.tac & 0b11 {
            0b00 => {
                timer_update = (prev_divider & (1 << 9) != 0) && (!(self.div & (1 << 9))) != 0;
            }
            0b01 => {
                timer_update = (prev_divider & (1 << 3) != 0) && (!(self.div & (1 << 3))) != 0;
            }
            0b10 => {
                timer_update = (prev_divider & (1 << 5) != 0) && (!(self.div & (1 << 5))) != 0;
            }
            0b11 => {
                timer_update = (prev_divider & (1 << 7) != 0) && (!(self.div & (1 << 7))) != 0;
            }
            _ => {}
        };

        if timer_update && self.is_timer_enabled() {
            self.tima += 1;

            if self.tima == 0xFF {
                self.tima = self.tma;
                self.set_interrupt(Interrupt::Timer);
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => {
                // DIV
                self.div = 0;
            },
            0xFF05 => {
                // TIMA
                self.tima = value;
            },
            0xFF06 => {
                // TMA
                self.tma = value;
            },
            0xFF07 => {
                // TAC
                self.tac = value;
            },
            _ => panic!("UNSUPPORTED TIMER WRITE: {:04X}, {:02X}", address, value)
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => {
                // DIV
                (self.div >> 8) as u8
            },
            0xFF05 => {
                // TIMA
                self.tima
            },
            0xFF06 => {
                // TMA
                self.tma
            },
            0xFF07 => {
                // TAC
                self.tac
            },
            _ => panic!("UNSUPPORTED TIMER READ: {:04X}", address)
        }
    }

    fn is_timer_enabled(&self) -> bool {
        (self.tac & (1 << 2)) != 0
    }

    fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupts |= interrupt as u8;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
