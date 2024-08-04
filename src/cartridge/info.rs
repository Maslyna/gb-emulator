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

#[derive(Debug)]
pub enum License {
    Unknown,
    None,
    Nintendo,
    Capcom,
    ElectronicArts,
    HudsonSoft,
    Bai,
    Kss,
    Pow,
    PcmComplete,
    SanX,
    KemcoJapan,
    Seta,
    Viacom,
    Bandai,
    Ocean,
    Konami,
    Hector,
    Taito,
    Hudson,
    Banpresto,
    Ubisoft,
    Atlus,
    Malibu,
    Andel,
    BulletProof,
    Irem,
    Absolute,
    Acclaim,
    Activision,
    AmericanSammy,
    HiTechEntertainment,
    Ljn,
    Matchbox,
    Mattel,
    MiltonBradley,
    Titus,
    Virgin,
    LucasArts,
    Infogrames,
    Interplay,
    Broderbund,
    Sculptured,
    Sci,
    Thq,
    Accolade,
    Misawa,
    Lozc,
    TokumaShotenIntermedia,
    TsukudaOriginal,
    Chunsoft,
    VideoSystem,
    Varie,
    Yonezawa,
    Kaneko,
    PackInSoft,
}

impl From<u8> for License {
    fn from(value: u8) -> Self {
        match value {
            0x00u8 => License::None,
            0x01u8 => License::Nintendo,
            0x08u8 => License::Capcom,
            0x13u8 => License::ElectronicArts,
            0x18u8 => License::HudsonSoft,
            0x19u8 => License::Bai,
            0x20u8 => License::Kss,
            0x22u8 => License::Pow,
            0x24u8 => License::PcmComplete,
            0x25u8 => License::SanX,
            0x28u8 => License::KemcoJapan,
            0x29u8 => License::Seta,
            0x30u8 => License::Viacom,
            0x31u8 => License::Nintendo,
            0x32u8 => License::Bandai,
            0x33u8 => License::Ocean,
            0x34u8 => License::Konami,
            0x35u8 => License::Hector,
            0x37u8 => License::Taito,
            0x38u8 => License::Hudson,
            0x39u8 => License::Banpresto,
            0x41u8 => License::Ubisoft,
            0x42u8 => License::Atlus,
            0x44u8 => License::Malibu,
            0x46u8 => License::Andel,
            0x47u8 => License::BulletProof,
            0x49u8 => License::Irem,
            0x50u8 => License::Absolute,
            0x51u8 => License::Acclaim,
            0x52u8 => License::Activision,
            0x53u8 => License::AmericanSammy,
            0x54u8 => License::Konami,
            0x55u8 => License::HiTechEntertainment,
            0x56u8 => License::Ljn,
            0x57u8 => License::Matchbox,
            0x58u8 => License::Mattel,
            0x59u8 => License::MiltonBradley,
            0x60u8 => License::Titus,
            0x61u8 => License::Virgin,
            0x64u8 => License::LucasArts,
            0x67u8 => License::Ocean,
            0x69u8 => License::ElectronicArts,
            0x70u8 => License::Infogrames,
            0x71u8 => License::Interplay,
            0x72u8 => License::Broderbund,
            0x73u8 => License::Sculptured,
            0x75u8 => License::Sci,
            0x78u8 => License::Thq,
            0x79u8 => License::Accolade,
            0x80u8 => License::Misawa,
            0x83u8 => License::Lozc,
            0x86u8 => License::TokumaShotenIntermedia,
            0x87u8 => License::TsukudaOriginal,
            0x91u8 => License::Chunsoft,
            0x92u8 => License::VideoSystem,
            0x93u8 => License::Ocean,
            0x95u8 => License::Varie,
            0x96u8 => License::Yonezawa,
            0x97u8 => License::Kaneko,
            0x99u8 => License::PackInSoft,
            0xA4u8 => License::Konami,
            _ => License::Unknown
        }
    }
}

impl From<u8> for RomType {
    fn from(value: u8) -> Self {
        match value {
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
