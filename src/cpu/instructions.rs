#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instruction {
    pub in_type: InstructionType,
    pub mode: AddressMode,
    pub reg_1: RegisterType,
    pub reg_2: RegisterType,
    pub condition: ConditionType,
    pub param: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AddressMode {
    Imp,
    R_D16,
    R_R,
    MR_R,
    R,
    R_D8,
    R_MR,
    R_HLI,
    R_HLD,
    HLI_R,
    HLD_R,
    R_A8,
    A8_R,
    HL_SPR,
    D16,
    D8,
    D16_R,
    MR_D8,
    MR,
    A16_R,
    R_A16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum RegisterType {
    None,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum InstructionType {
    None,
    Nop,
    Ld,
    Inc,
    Dec,
    Rlca,
    Add,
    Rrca,
    Stop,
    Rla,
    Jr,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
    Pop,
    Jp,
    Push,
    Ret,
    Cb,
    Call,
    Reti,
    Ldh,
    Jphl,
    Di,
    Ei,
    Rst,
    Err,
    //CB instructions...
    Rlc,
    Rrc,
    RL,
    Rr,
    Sla,
    Sra,
    Swap,
    Srl,
    Bit,
    Res,
    Set,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}

impl Instruction {
    const INSTRUCTIONS: [Instruction; 0x100] = Instruction::init_instructions();

    pub const fn default() -> Self {
        Self {
            in_type: InstructionType::None,
            mode: AddressMode::Imp,
            reg_1: RegisterType::None,
            reg_2: RegisterType::None,
            condition: ConditionType::None,
            param: 0,
        }
    }

    const fn init_instructions() -> [Instruction; 0x100] {
        use AddressMode as AM;
        use InstructionType as IT;
        use RegisterType as RT;

        let mut inst: [Instruction; 0x100] = [Instruction::default(); 0x100];

        // 0x0X
        inst[0x00] = Instruction {
            in_type: IT::Nop,
            ..Instruction::default()
        };
        inst[0x01] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D16,
            reg_1: RT::BC,
            ..Instruction::default()
        };
        inst[0x02] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::BC,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x05] = Instruction {
            in_type: IT::Dec,
            mode: AM::R,
            reg_1: RT::B,
            ..Instruction::default()
        };
        inst[0x06] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::B,
            ..Instruction::default()
        };
        inst[0x08] = Instruction {
            in_type: IT::Ld,
            mode: AM::A16_R,
            reg_2: RT::SP,
            ..Instruction::default()
        };
        inst[0x0A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::A,
            reg_2: RT::BC,
            ..Instruction::default()
        };
        inst[0x0E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::C,
            ..Instruction::default()
        };

        // 0x1X
        inst[0x11] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D16,
            reg_1: RT::DE,
            ..Instruction::default()
        };
        inst[0x12] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::DE,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x15] = Instruction {
            in_type: IT::Dec,
            mode: AM::R,
            reg_1: RT::D,
            ..Instruction::default()
        };
        inst[0x16] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::D,
            ..Instruction::default()
        };
        inst[0x1A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::A,
            reg_2: RT::DE,
            ..Instruction::default()
        };
        inst[0x1E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::E,
            ..Instruction::default()
        };

        //0x2X
        inst[0x21] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D16,
            reg_1: RT::HL,
            ..Instruction::default()
        };
        inst[0x22] = Instruction {
            in_type: IT::Ld,
            mode: AM::HLI_R,
            reg_1: RT::HL,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x25] = Instruction {
            in_type: IT::Dec,
            mode: AM::R,
            reg_1: RT::H,
            ..Instruction::default()
        };
        inst[0x26] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::H,
            ..Instruction::default()
        };
        inst[0x2A] = Instruction {
            in_type: IT::Ld,
            mode: AM::HLI_R,
            reg_1: RT::A,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x2E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::L,
            ..Instruction::default()
        };

        //0x3X
        inst[0x31] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D16,
            reg_1: RT::SP,
            ..Instruction::default()
        };
        inst[0x32] = Instruction {
            in_type: IT::Ld,
            mode: AM::HLD_R,
            reg_1: RT::HL,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x35] = Instruction {
            in_type: IT::Dec,
            mode: AM::R,
            reg_1: RT::HL,
            ..Instruction::default()
        };
        inst[0x36] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_D8,
            reg_1: RT::HL,
            ..Instruction::default()
        };
        inst[0x3A] = Instruction {
            in_type: IT::Ld,
            mode: AM::HLD_R,
            reg_1: RT::A,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x3E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_D8,
            reg_1: RT::A,
            ..Instruction::default()
        };

        //0x4X
        inst[0x40] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x41] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x42] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x43] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x44] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x45] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x46] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::B,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x47] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::B,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x48] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x49] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x4A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x4B] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x4C] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x4D] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x4E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::C,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x4F] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::C,
            reg_2: RT::A,
            ..Instruction::default()
        };

        //0x5X
        inst[0x50] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x51] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x52] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x53] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x54] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x55] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::D,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x56] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::D,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x57] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x58] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x59] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x5A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x5B] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x5C] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x5D] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x5E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::E,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x5F] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::E,
            reg_2: RT::A,
            ..Instruction::default()
        };

        //0x6X
        inst[0x60] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x61] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x62] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x63] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x64] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x65] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x66] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::H,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x67] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::H,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x68] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x69] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x6A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x6B] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x6C] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x6D] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x6E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::L,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x6F] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::L,
            reg_2: RT::A,
            ..Instruction::default()
        };

        //0x7X
        inst[0x70] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x71] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x72] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x73] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x74] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x75] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x76] = Instruction {
            in_type: IT::Halt,
            ..Instruction::default()
        };
        inst[0x77] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::HL,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0x78] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::B,
            ..Instruction::default()
        };
        inst[0x79] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0x7A] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::D,
            ..Instruction::default()
        };
        inst[0x7B] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::E,
            ..Instruction::default()
        };
        inst[0x7C] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::H,
            ..Instruction::default()
        };
        inst[0x7D] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::L,
            ..Instruction::default()
        };
        inst[0x7E] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::A,
            reg_2: RT::HL,
            ..Instruction::default()
        };
        inst[0x7F] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_R,
            reg_1: RT::A,
            reg_2: RT::A,
            ..Instruction::default()
        };

        //0xAX
        inst[0xAF] = Instruction {
            in_type: IT::Xor,
            mode: AM::R,
            reg_1: RT::A,
            ..Instruction::default()
        };

        //0xCX
        inst[0xC1] = Instruction {
            in_type: IT::Pop,
            mode: AM::R,
            reg_1: RT::BC,
            ..Instruction::default()
        };
        inst[0xC3] = Instruction {
            in_type: IT::Jp,
            mode: AM::D16,
            ..Instruction::default()
        };
        inst[0xC5] = Instruction {
            in_type: IT::Push,
            mode: AM::R,
            reg_1: RT::BC,
            ..Instruction::default()
        };

        //0xD1
        inst[0xD1] = Instruction {
            in_type: IT::Pop,
            mode: AM::R,
            reg_1: RT::DE,
            ..Instruction::default()
        };
        inst[0xD5] = Instruction {
            in_type: IT::Push,
            mode: AM::R,
            reg_1: RT::DE,
            ..Instruction::default()
        };


        //0xEX
        inst[0xE0] = Instruction {
            in_type: IT::Ldh,
            mode: AM::A8_R,
            reg_1: RT::None,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0xE1] = Instruction {
            in_type: IT::Pop,
            mode: AM::R,
            reg_1: RT::HL,
            ..Instruction::default()
        };
        inst[0xE2] = Instruction {
            in_type: IT::Ld,
            mode: AM::MR_R,
            reg_1: RT::C,
            reg_2: RT::A,
            ..Instruction::default()
        };
        inst[0xE5] = Instruction {
            in_type: IT::Push,
            mode: AM::R,
            reg_1: RT::HL,
            ..Instruction::default()
        };
        inst[0xEA] = Instruction {
            in_type: IT::Ld,
            mode: AM::A16_R,
            reg_1: RT::None,
            reg_2: RT::A,
            ..Instruction::default()
        };

        //0xFX
        inst[0xF0] = Instruction {
            in_type: IT::Ldh,
            mode: AM::R_A8,
            reg_1: RT::A,
            ..Instruction::default()
        };
        inst[0xF1] = Instruction {
            in_type: IT::Pop,
            mode: AM::R,
            reg_1: RT::AF,
            ..Instruction::default()
        };
        inst[0xF2] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_MR,
            reg_1: RT::A,
            reg_2: RT::C,
            ..Instruction::default()
        };
        inst[0xF3] = Instruction {
            in_type: IT::Di,
            ..Instruction::default()
        };
        inst[0xF5] = Instruction {
            in_type: IT::Push,
            mode: AM::R,
            reg_1: RT::AF,
            ..Instruction::default()
        };
        inst[0xFA] = Instruction {
            in_type: IT::Ld,
            mode: AM::R_A16,
            reg_1: RT::A,
            ..Instruction::default()
        };

        return inst;
    }
}

impl RegisterType {
    pub fn is_16bit(&self) -> bool {
        use RegisterType as RT;
        matches!(self, RT::AF | RT::BC | RT::DE | RT::HL | RT::SP | RT::PC)
    }
}

impl From<u8> for Instruction {
    fn from(code: u8) -> Self {
        Instruction::INSTRUCTIONS[code as usize]
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::default()
    }
}
