use super::Bus;

#[derive(Debug)]
pub struct Dma {
    is_active: bool,
    byte: u8,
    value: u8,
    start_delay: u8,
}

impl Dma {
    pub const fn new() -> Self {
        Self {
            is_active: false,
            byte: 0,
            value: 0,
            start_delay: 0,
        }
    }

    pub fn start(&mut self, start: u8) {
        self.is_active = true;
        self.byte = 0;
        self.value = start;
        self.start_delay = 2;
    }

    pub fn is_transfering(&self) -> bool {
        self.is_active
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        if !self.is_active {
            return;
        }

        if self.start_delay > 0 {
            self.start_delay -= 1;
            return;
        }

        let data = bus.read(
            (self.value as u16)
                .wrapping_mul(0x100)
                .wrapping_add(self.byte as u16),
        );
        bus.ppu.oam_write(self.byte as u16, data);

        self.byte = self.byte.wrapping_add(1);
        self.is_active = self.byte < 0xA0;
    }
}