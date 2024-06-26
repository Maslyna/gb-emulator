use phf::phf_map;
use std::fs::File;
use std::io::{self, Read};
use std::result::Result;

pub fn load_cart(path: &str) -> Result<RomHeader, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let mut buffer: [u8; 0x150] = [0; 0x150];
    reader.read_exact(&mut buffer)?;

    let header = RomHeader {
        entry: [buffer[0], buffer[1], buffer[2], buffer[3]],
        logo: {
            let mut logo = [0u8; 0x30];
            logo.copy_from_slice(&buffer[0x04..0x34]);
            logo
        },
        title: {
            let mut title = [0u8; 16];
            title.copy_from_slice(&buffer[0x34..0x44]);
            title
        },
        manufacture_code: {
            let mut code = [0u8; 4];
            code.copy_from_slice(&buffer[0x44..0x48]);
            code
        },
        new_licence_code: {
            let mut code = [0u8; 2];
            code.copy_from_slice(&buffer[0x48..0x4A]);
            code
        },
        license_code: buffer[0x4B],
        dest_code: buffer[0x4C],
        cgb_flag: buffer[0x4D],
        sgb_flag: buffer[0x4E],
        cart_type: buffer[0x4F],
        rom_size: buffer[0x50],
        ram_size: buffer[0x51],
        checksum: buffer[0x52],
        global_checksum: [buffer[0x53], buffer[0x54]],
    };

    return Ok(header);
}

#[derive(Debug)]
pub struct RomHeader {
    pub entry: [u8; 4],
    pub logo: [u8; 0x30],
    pub title: [u8; 16],
    pub manufacture_code: [u8; 4],
    pub new_licence_code: [u8; 2],
    pub license_code: u8,
    pub dest_code: u8,
    pub cgb_flag: u8,
    pub sgb_flag: u8,
    pub cart_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub checksum: u8,
    pub global_checksum: [u8; 2],
}

pub struct CartContext {
    pub filename: String,
    pub rom_size: Box<u32>,
    pub rom_data: *const u8,
    pub rom_header: *const u8
}


pub const ROM_TYPE: phf::Map<u8, &'static str> = phf_map! {
    0x00u8 => "ROM ONLY",
    0x01u8 => "MBC1",
    0x02u8 => "MBC1+RAM",
    0x03u8 => "MBC1+RAM+BATTERY",
    0x05u8 => "MBC2",
    0x06u8 => "MBC2+BATTERY",
    0x08u8 => "ROM+RAM",
    0x09u8 => "ROM+RAM+BATTERY",
    0x0Bu8 => "MMM01",
    0x0Cu8 => "MMM01+RAM",
    0x0Du8 => "MMM01+RAM+BATTERY",
    0x0Fu8 => "MBC3+TIMER+BATTERY",
    0x10u8 => "MBC3+TIMER+RAM+BATTERY",
    0x11u8 => "MBC3",
    0x12u8 => "MBC3+RAM",
    0x13u8 => "MBC3+RAM+BATTERY",
    0x19u8 => "MBC5",
    0x1Au8 => "MBC5+RAM",
    0x1Bu8 => "MBC5+RAM+BATTERY",
    0x1Cu8 => "MBC5+RUMBLE",
    0x1Du8 => "MBC5+RUMBLE+RAM",
    0x1Eu8 => "MBC5+RUMBLE+RAM+BATTERY",
    0x20u8 => "MBC6",
    0x22u8 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
    0xFCu8 => "POCKET CAMERA",
    0xFDu8 => "BANDAI TAMA5",
    0xFEu8 => "HuC3",
    0xFFu8 => "HuC1+RAM+BATTERY",
};

pub const LIC_CODE: phf::Map<u8, &'static str> = phf_map! {
    0x00u8 => "None",
    0x01u8 => "Nintendo R&D1",
    0x08u8 => "Capcom",
    0x13u8 => "Electronic Arts",
    0x18u8 => "Hudson Soft",
    0x19u8 => "b-ai",
    0x20u8 => "kss",
    0x22u8 => "pow",
    0x24u8 => "PCM Complete",
    0x25u8 => "san-x",
    0x28u8 => "Kemco Japan",
    0x29u8 => "seta",
    0x30u8 => "Viacom",
    0x31u8 => "Nintendo",
    0x32u8 => "Bandai",
    0x33u8 => "Ocean/Acclaim",
    0x34u8 => "Konami",
    0x35u8 => "Hector",
    0x37u8 => "Taito",
    0x38u8 => "Hudson",
    0x39u8 => "Banpresto",
    0x41u8 => "Ubi Soft",
    0x42u8 => "Atlus",
    0x44u8 => "Malibu",
    0x46u8 => "angel",
    0x47u8 => "Bullet-Proof",
    0x49u8 => "irem",
    0x50u8 => "Absolute",
    0x51u8 => "Acclaim",
    0x52u8 => "Activision",
    0x53u8 => "American sammy",
    0x54u8 => "Konami",
    0x55u8 => "Hi tech entertainment",
    0x56u8 => "LJN",
    0x57u8 => "Matchbox",
    0x58u8 => "Mattel",
    0x59u8 => "Milton Bradley",
    0x60u8 => "Titus",
    0x61u8 => "Virgin",
    0x64u8 => "LucasArts",
    0x67u8 => "Ocean",
    0x69u8 => "Electronic Arts",
    0x70u8 => "Infogrames",
    0x71u8 => "Interplay",
    0x72u8 => "Broderbund",
    0x73u8 => "sculptured",
    0x75u8 => "sci",
    0x78u8 => "THQ",
    0x79u8 => "Accolade",
    0x80u8 => "misawa",
    0x83u8 => "lozc",
    0x86u8 => "Tokuma Shoten Intermedia",
    0x87u8 => "Tsukuda Original",
    0x91u8 => "Chunsoft",
    0x92u8 => "Video system",
    0x93u8 => "Ocean/Acclaim",
    0x95u8 => "Varie",
    0x96u8 => "Yonezawa/s’pal",
    0x97u8 => "Kaneko",
    0x99u8 => "Pack in soft",
    0xA4u8 => "Konami (Yu-Gi-Oh!)"
};

pub enum RomType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
    Mbc3TimerBattery = 0x0F,
    Mbc3TimerRamBattery = 0x10,
    Mbc3 = 0x11,
    Mbc3Ram = 0x12,
    Mbc3RamBattery = 0x13,
    Mbc5 = 0x19,
    Mbc5Ram = 0x1A,
    Mbc5RamBattery = 0x1B,
    Mbc5Rumble = 0x1C,
    Mbc5RumbleRam = 0x1D,
    Mbc5RumbleRamBattery = 0x1E,
    Mbc6 = 0x20,
    Mbc7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
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
            _ => panic!("Invalid rom type provided: {:#04x}", byte),
        }
    }
}