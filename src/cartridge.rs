use phf::phf_map;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn load_cart(path: String) -> Result<Rom, CartrigeError> {
    let mut file: File = match File::open(&path) {
        Ok(f) => f,
        Err(err) => return Err(CartrigeError::IoError(err)),
    };

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let header = RomHeader::read(&buffer);

    let rom = Rom {
        filename: path,
        rom_data: buffer.into_boxed_slice(),
        rom_header: header,
    };

    return match rom.is_checksum_valid() {
        true => Ok(rom),
        false => Err(CartrigeError::InvalidFile("Invalid checksum")),
    };
}

#[derive(Debug)]
pub enum CartrigeError {
    InvalidFile(&'static str),
    IoError(io::Error),
}

impl std::fmt::Display for CartrigeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFile(msg) => writeln!(f, "InvalidFile({})", msg),
            Self::IoError(err) => writeln!(f, "IoError({})", err),
        }
    }
}

impl std::error::Error for CartrigeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CartrigeError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

pub struct Rom {
    filename: String,
    rom_data: Box<[u8]>,
    rom_header: RomHeader,
}

impl Rom {
    fn calculate_cecksum(&self) -> u8 {
        let mut x: u16 = 0;
        for i in 0x0134..=0x014C {
            x = x.wrapping_sub(self.rom_data[i] as u16).wrapping_sub(1);
        }
        return x as u8;
    }

    fn is_checksum_valid(&self) -> bool {
        return self.calculate_cecksum() == self.rom_header.checksum;
    }
}

impl std::fmt::Display for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ROM:")?;
        writeln!(f, "Filename:")?;
        writeln!(f, "Size: {} KB", 32 << self.rom_header.rom_size)?;
        writeln!(f, "Checksum: {:02X}", self.calculate_cecksum())?;
        writeln!(f, "Header: [\n{}]", self.rom_header)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RomHeader {
    entry: [u8; 4],            // 0x100 - 0x103
    logo: [u8; 0x30],          // 0x104 - 0x133
    title: [u8; 16],           // 0x134 - 0x143
    manufacture_code: [u8; 4], // 0x13F - 0x142
    new_licence_code: [u8; 2], // 0x144 - 0x145
    license_code: u8,          // 0x14A
    dest_code: u8,             // 0x14B
    cgb_flag: u8,              // 0x143
    sgb_flag: u8,              // 0x146
    cart_type: u8,             // 0x147
    rom_size: u8,              // 0x148
    ram_size: u8,              // 0x149
    checksum: u8,              // 0x14D
    global_checksum: [u8; 2],  // 0x14E - 0x14F
}

impl RomHeader {
    pub fn read(cartrige: &[u8]) -> Self {
        Self {
            entry: cartrige[0x100..=0x103].try_into().unwrap(),
            logo: cartrige[0x104..=0x133].try_into().unwrap(),
            title: cartrige[0x134..=0x143].try_into().unwrap(),
            manufacture_code: cartrige[0x13F..=0x142].try_into().unwrap(),
            new_licence_code: cartrige[0x144..=0x145].try_into().unwrap(),
            license_code: cartrige[0x14A],
            dest_code: cartrige[0x14B],
            cgb_flag: cartrige[0x143],
            sgb_flag: cartrige[0x146],
            cart_type: cartrige[0x147],
            rom_size: cartrige[0x148],
            ram_size: cartrige[0x149],
            checksum: cartrige[0x14D],
            global_checksum: cartrige[0x14E..=0x14F].try_into().unwrap(),
        }
    }

    pub fn license_name(&self) -> &'static str {
        return LIC_CODE
            .get(&self.license_code)
            .copied()
            .unwrap_or("UNKNOWN");
    }

    pub fn rom_type_name(&self) -> &'static str {
        return ROM_TYPE.get(&self.cart_type).copied().unwrap_or("UNKNOWN");
    }
}

impl std::fmt::Display for RomHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\tEntry: {:02X?}", self.entry)?;
        writeln!(f, "\tLogo: {:02X?}", self.logo)?;
        writeln!(f, "\tTitle: {}", u8_slice_to_ascii(&self.title))?;
        writeln!(f, "\tManufacture Code: {:?}", self.manufacture_code)?;
        writeln!(f, "\tNew License Code: {:02X?}", self.new_licence_code)?;
        writeln!(
            f,
            "\tLicense Code: {:02X} - {}",
            self.license_code,
            self.license_name()
        )?;
        writeln!(f, "\tDestination Code: {:02X}", self.dest_code)?;
        writeln!(f, "\tCGB Flag: {:02X}", self.cgb_flag)?;
        writeln!(f, "\tSGB Flag: {:02X}", self.sgb_flag)?;
        writeln!(
            f,
            "\tROM Type: {:02X} - {}",
            self.cart_type,
            self.rom_type_name()
        )?;
        writeln!(f, "\tROM Size: {:02X}", self.rom_size)?;
        writeln!(f, "\tRAM Size: {:02X}", self.ram_size)?;
        writeln!(f, "\tChecksum: {:02X}", self.checksum)?;
        writeln!(f, "\tGlobal Checksum: {:02X?}", self.global_checksum)?;
        Ok(())
    }
}

pub struct CartContext {
    pub filename: String,
    pub rom_size: u32,
    pub rom_data: Vec<u8>,
    pub rom_header: Vec<u8>,
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

fn u8_slice_to_ascii(slice: &[u8]) -> String {
    let s = slice.iter().map(|byte| *byte as char).collect::<String>();
    return s;
}
