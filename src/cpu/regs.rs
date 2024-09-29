#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub const fn new() -> Self {
        Self {
            a: 0x01,
            f: 0x80,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x100,
            sp: 0xFFFE,
        }
    }

    pub fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z >= 0 {
            set_bit!(self.f, 7, z != 0);
        }
        if n >= 0 {
            set_bit!(self.f, 6, n != 0);
        }
        if h >= 0 {
            set_bit!(self.f, 5, h != 0);
        }
        if c >= 0 {
            set_bit!(self.f, 4, c != 0);
        }
    }

    #[inline(always)]
    pub fn flag_z(&self) -> bool {
        bit!(self.f, 7)
    }

    #[inline(always)]
    pub fn flag_n(&self) -> bool {
        bit!(self.f, 6)
    }

    #[inline(always)]
    pub fn flag_h(&self) -> bool {
        bit!(self.f, 5)
    }

    #[inline(always)]
    pub fn flag_c(&self) -> bool {
        bit!(self.f, 4)
    }
}
