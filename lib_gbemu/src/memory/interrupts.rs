use crate::cpu::Cpu;
use crate::memory::Bus;

pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Interrupt {
    VBlank = 0x01,
    LcdStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

impl Interrupt {
    const fn address(self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LcdStat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
        }
    }
}

#[derive(Debug)]
pub struct InterruptState {
    pub flags: u8,
    pub enabled: u8,
}

impl InterruptState {
    pub const fn new() -> Self {
        Self {
            flags: 0,
            enabled: 0,
        }
    }

    pub fn enable_flag(&mut self, interrupt: Interrupt) {
        self.flags |= interrupt as u8;
    }

    pub fn is_active(&self, interrupt: Interrupt) -> bool {
        let interrupt = interrupt as u8;

        self.flags & interrupt == interrupt && self.enabled & interrupt == interrupt
    }

    pub fn remove_flag(&mut self, interrupt: Interrupt) {
        self.flags &= !(interrupt as u8);
    }

    pub fn has_any_flag(&self) -> bool {
        (self.flags & self.enabled) != 0
    }
}

impl Default for InterruptState {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn handle_interrupts(&mut self, bus: &mut Bus) {
        match () {
            _ if self.process_interrupt(bus, Interrupt::VBlank) => (),
            _ if self.process_interrupt(bus, Interrupt::LcdStat) => (),
            _ if self.process_interrupt(bus, Interrupt::Timer) => (),
            _ if self.process_interrupt(bus, Interrupt::Serial) => (),
            _ if self.process_interrupt(bus, Interrupt::Joypad) => (),
            _ => (),
        }
    }

    fn process_interrupt(&mut self, bus: &mut Bus, interrupt: Interrupt) -> bool {
        let address = interrupt.address();

        if !bus.interrupts.is_active(interrupt) {
            return false;
        }

        self.stack_push16(self.regs.pc, bus);
        self.regs.pc = address;

        bus.interrupts.remove_flag(interrupt);
        self.is_halted = false;
        self.interrupt_master_enabled = false;

        true
    }
}
