use crate::cpu::Cpu;
use crate::memory::Bus;

pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const _INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;

#[derive(Clone, Copy)]
pub enum Interrupt {
    Vblank = 0x01,
    LcdStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

impl Interrupt {
    fn address(self) -> u16 {
        match self {
            Interrupt::Vblank => 0x0040,
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
        let it = interrupt as u8;

        assert_eq!(
            (self.flags & it == it) && (self.enabled & it == it),
            (self.flags & it != 0) && (self.enabled & it != 0)
        );

        (self.flags & it == it) && (self.enabled & it == it)
    }

    pub fn remove_flag(&mut self, interrupt: Interrupt) {
        let mut flags_copy = self.flags;
        flags_copy &= interrupt as u8 ^ 0xFF;

        self.flags &= !(interrupt as u8);

        assert_eq!(flags_copy, self.flags)
    }
}

impl Default for InterruptState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn handle_interrupts(cpu: &mut Cpu, bus: &mut Bus) {
    const INTERRUPTS: [Interrupt; 5] = [
        Interrupt::Vblank,
        Interrupt::LcdStat,
        Interrupt::Timer,
        Interrupt::Serial,
        Interrupt::Joypad,
    ];
    for interrupt in INTERRUPTS {
        if handle_interrupt(cpu, bus, interrupt, interrupt.address()) {
            return;
        }
    }
}

fn handle_interrupt(cpu: &mut Cpu, bus: &mut Bus, interrupt: Interrupt, address: u16) -> bool {
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
