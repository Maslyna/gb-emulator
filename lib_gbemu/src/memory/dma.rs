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
}

impl Bus {
    pub fn dma_tick(&mut self) {
        if !self.dma.is_active {
            return;
        }

        if self.dma.start_delay > 0 {
            self.dma.start_delay -= 1;
            return;
        }

        let data = self.read(
            (self.dma.value as u16)
                .wrapping_mul(0x100)
                .wrapping_add(self.dma.byte as u16),
        );
        self.ppu.oam_write(self.dma.byte as u16, data);

        self.dma.byte = self.dma.byte.wrapping_add(1);
        self.dma.is_active = self.dma.byte < 0xA0;
    }
}