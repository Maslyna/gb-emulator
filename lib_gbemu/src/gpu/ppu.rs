#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Oam {
    y: u8,
    x: u8,
    title: u8,
    flags: u8,
}

impl Oam {
    pub const fn new() -> Self {
        Self {
            y: 0,
            x: 0,
            title: 0,
            flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct Ppu {
    oam_ram: [Oam; 0x40],
    vram: [u8; 0x2000],
}

impl Ppu {
    pub const fn new() -> Self {
        Self {
            oam_ram: [Oam::new(); 0x40],
            vram: [0; 0x2000],
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
        let index = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        let ptr = self.oam_ram.as_mut_ptr() as *mut u8;
        unsafe {
            *ptr.add(index) = value;
        }
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        let index = if address >= 0xFE00 {
            address.wrapping_sub(0xFE00)
        } else {
            address
        } as usize;

        let ptr = self.oam_ram.as_ptr() as *const u8;
        unsafe { *ptr.add(index) }
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        self.vram[address.wrapping_sub(0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[address.wrapping_sub(0x8000) as usize]
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
