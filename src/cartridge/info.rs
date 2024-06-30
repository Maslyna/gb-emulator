pub const LOCATION_ENTRY_START: usize = 0x100;
pub const LOCATION_ENTRY_END: usize = 0x103;
pub const LOCATION_LOGO_START: usize = 0x104;
pub const LOCATION_LOGO_END: usize = 0x133;
pub const LOCATION_TITLE_START: usize = 0x134;
pub const LOCATION_TITLE_END: usize = 0x143;
pub const LOCATION_MANUFACTURE_START: usize = 0x13F;
pub const LOCATION_MANUFACTURE_END: usize = 0x142;
pub const LOCATION_NEW_LICENCE_CODE_START: usize = 0x144;
pub const LOCATION_NEW_LICENCE_CODE_END: usize = 0x145;
pub const LOCATION_LICENSE_CODE: usize = 0x14A;
pub const LOCATION_DEST_CODE: usize = 0x14B;
pub const LOCATION_CGB_FLAG: usize = 0x143;
pub const LOCATION_SGB_FLAG: usize = 0x146;
pub const LOCATION_CART_TYPE: usize = 0x147;
pub const LOCATION_ROM_SIZE: usize = 0x148;
pub const LOCATION_RAM_SIZE: usize = 0x149;
pub const LOCATION_CHECKSUM: usize = 0x14D;
pub const LOCATION_G_CHECKSUM_START: usize = 0x14E;
pub const LOCATION_G_CHECKSUM_END: usize = 0x14F;

#[derive(Debug)]
pub enum RomType {
    Unknown,
    RomOnly,
    Mbc1,
    Mbc1Ram,
    Mbc1RamBattery,
    Mbc2,
    Mbc2Battery,
    RomRam,
    RomRamBattery,
    Mmm01,
    Mmm01Ram,
    Mmm01RamBattery,
    Mbc3TimerBattery,
    Mbc3TimerRamBattery,
    Mbc3,
    Mbc3Ram,
    Mbc3RamBattery,
    Mbc5,
    Mbc5Ram,
    Mbc5RamBattery,
    Mbc5Rumble,
    Mbc5RumbleRam,
    Mbc5RumbleRamBattery,
    Mbc6,
    Mbc7SensorRumbleRamBattery,
    PocketCamera,
    BandaiTama5,
    HuC3,
    HuC1RamBattery,
}

impl RomType {
    pub fn from_byte(byte: u8) -> RomType {
        match byte {
            0x00 => RomType::RomOnly,
            0x01 => RomType::Mbc1,
            0x02 => RomType::Mbc1Ram,
            0x03 => RomType::Mbc1RamBattery,
            0x05 => RomType::Mbc2,
            0x06 => RomType::Mbc2Battery,
            0x08 => RomType::RomRam,
            0x09 => RomType::RomRamBattery,
            0x0B => RomType::Mmm01,
            0x0C => RomType::Mmm01Ram,
            0x0D => RomType::Mmm01RamBattery,
            0x0F => RomType::Mbc3TimerBattery,
            0x10 => RomType::Mbc3TimerRamBattery,
            0x11 => RomType::Mbc3,
            0x12 => RomType::Mbc3Ram,
            0x13 => RomType::Mbc3RamBattery,
            0x19 => RomType::Mbc5,
            0x1A => RomType::Mbc5Ram,
            0x1B => RomType::Mbc5RamBattery,
            0x1C => RomType::Mbc5Rumble,
            0x1D => RomType::Mbc5RumbleRam,
            0x1E => RomType::Mbc5RumbleRamBattery,
            0x20 => RomType::Mbc6,
            0x22 => RomType::Mbc7SensorRumbleRamBattery,
            0xFC => RomType::PocketCamera,
            0xFD => RomType::BandaiTama5,
            0xFE => RomType::HuC3,
            0xFF => RomType::HuC1RamBattery,
            _ => RomType::Unknown,
        }
    }
}