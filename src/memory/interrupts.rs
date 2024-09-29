pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
pub const _INTERRUPT_FLAGS_ADDRESS: u16 = 0xFF0F;


#[derive(Debug)]
pub struct Interrupt {
    pub _flags: u8,
    pub enabled: u8,
}

impl Interrupt {
    pub const fn new() -> Interrupt {
        Interrupt {
            _flags: 0,
            enabled: 0,
        }
    }
}