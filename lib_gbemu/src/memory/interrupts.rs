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
    fn address(self) -> u16 {
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
        self.is_active(Interrupt::Timer)
            || self.is_active(Interrupt::VBlank)
            || self.is_active(Interrupt::Joypad)
            || self.is_active(Interrupt::LcdStat)
            || self.is_active(Interrupt::Serial)
    }
}

impl Default for InterruptState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn handle(cpu: &mut Cpu, bus: &mut Bus) {
    match () {
        _ if check(cpu, bus, Interrupt::VBlank) => (),
        _ if check(cpu, bus, Interrupt::LcdStat) => (),
        _ if check(cpu, bus, Interrupt::Timer) => (),
        _ if check(cpu, bus, Interrupt::Serial) => (),
        _ if check(cpu, bus, Interrupt::Joypad) => (),
        _ => (),
    }
}

fn check(cpu: &mut Cpu, bus: &mut Bus, interrupt: Interrupt) -> bool {
    let address = interrupt.address();

    if !bus.interrupts.is_active(interrupt) {
        return false;
    }

    cpu.stack_push16(cpu.regs.pc, bus);
    cpu.regs.pc = address;

    bus.interrupts.remove_flag(interrupt);
    cpu.is_halted = false;
    cpu.interrupt_master_enabled = false;

    true
}
