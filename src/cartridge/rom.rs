//
//  TODO: probably I should rewrite here everything
//
use super::info::*;

use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum CartrigeError {
    InvalidFile(&'static str),
    IoError(io::Error),
}
pub struct Rom {
    rom_data: Box<[u8]>,
    rom_header: Header,
}

pub struct Header {
    _entry: [u8; 4],            // 0x100 - 0x103
    _logo: [u8; 0x30],          // 0x104 - 0x133
    title: [u8; 16],           // 0x134 - 0x143
    manufacture_code: [u8; 4], // 0x13F - 0x142
    new_licence_code: [u8; 2], // 0x144 - 0x145
    license_code: u8,          // 0x14A
    dest_code: u8,             // 0x14B
    cgb_flag: u8,              // 0x143
    sgb_flag: u8,              // 0x146
    cart_type: RomType,        // 0x147
    rom_size: u8,              // 0x148
    ram_size: u8,              // 0x149
    checksum: u8,              // 0x14D
    global_checksum: [u8; 2],  // 0x14E - 0x14F
}

impl Rom {
    pub fn load(path: String) -> Result<Rom, CartrigeError> {
        let mut file: File = match File::open(path) {
            Ok(f) => f,
            Err(err) => return Err(CartrigeError::IoError(err)),
        };

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let header = Header::new(&buffer);

        let rom = Rom {
            rom_data: buffer.into_boxed_slice(),
            rom_header: header,
        };

        return match rom.is_checksum_valid() {
            true => Ok(rom),
            false => Err(CartrigeError::InvalidFile("Invalid checksum")),
        };
    }

    pub fn read(&self, address: u16) -> u8 {
        return self.rom_data[address as usize];
    }

    pub fn write(&mut self, _address: u16, _value: u8) {
        todo!("for now ROM only");
    }

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

impl Header {
    pub fn new(cartrige: &[u8]) -> Self {
        Self {
            _entry: cartrige[LOCATION_ENTRY_START..=LOCATION_ENTRY_END]
                .try_into()
                .unwrap(),
            _logo: cartrige[LOCATION_LOGO_START..=LOCATION_LOGO_END]
                .try_into()
                .unwrap(),
            title: cartrige[LOCATION_TITLE_START..=LOCATION_TITLE_END]
                .try_into()
                .unwrap(),
            manufacture_code: cartrige[LOCATION_MANUFACTURE_START..=LOCATION_MANUFACTURE_END]
                .try_into()
                .unwrap(),
            new_licence_code: cartrige
                [LOCATION_NEW_LICENCE_CODE_START..=LOCATION_NEW_LICENCE_CODE_END]
                .try_into()
                .unwrap(),
            license_code: cartrige[LOCATION_LICENSE_CODE],
            dest_code: cartrige[LOCATION_DEST_CODE],
            cgb_flag: cartrige[LOCATION_CGB_FLAG],
            sgb_flag: cartrige[LOCATION_SGB_FLAG],
            cart_type: RomType::from(cartrige[LOCATION_CART_TYPE]),
            rom_size: cartrige[LOCATION_ROM_SIZE],
            ram_size: cartrige[LOCATION_RAM_SIZE],
            checksum: cartrige[LOCATION_CHECKSUM],
            global_checksum: cartrige[LOCATION_G_CHECKSUM_START..=LOCATION_G_CHECKSUM_END]
                .try_into()
                .unwrap(),
        }
    }
}

impl std::fmt::Display for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ROM:")?;
        writeln!(f, "Size: {} KB", 32 << self.rom_header.rom_size)?;
        writeln!(f, "Checksum: {:02X}", self.calculate_cecksum())?;
        writeln!(f, "Header: [\n{}]", self.rom_header)?;
        Ok(())
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\tTitle: {}\n\
             \tManufacture Code: {:?}\n\
             \tNew License Code: {:02X?}\n\
             \tLicense Code: {:02X} - {:?}\n\
             \tDestination Code: {:02X}\n\
             \tCGB Flag: {:02X}\n\
             \tSGB Flag: {:02X}\n\
             \tROM Type: {:?}\n\
             \tROM Size: {:02X}\n\
             \tRAM Size: {:02X}\n\
             \tChecksum: {:02X}\n\
             \tGlobal Checksum: {:02X?}",
            u8_slice_to_ascii(&self.title),
            self.manufacture_code,
            self.new_licence_code,
            self.license_code,
            License::from(self.license_code),
            self.dest_code,
            self.cgb_flag,
            self.sgb_flag,
            self.cart_type,
            self.rom_size,
            self.ram_size,
            self.checksum,
            self.global_checksum,
        )
    }
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

fn u8_slice_to_ascii(slice: &[u8]) -> String {
    let s = slice.iter().map(|byte| *byte as char).collect::<String>();
    return s;
}
