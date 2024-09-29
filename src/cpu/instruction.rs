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
    Rl,
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
        use ConditionType as CT;
        use InstructionType as IT;
        use RegisterType as RT;

        let mut inst: [Instruction; 0x100] = [Instruction::default(); 0x100];

        //0x0X
        {
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
            inst[0x03] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::BC,
                ..Instruction::default()
            };
            inst[0x04] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::B,
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
            inst[0x09] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::HL,
                reg_2: RT::BC,
                ..Instruction::default()
            };
            inst[0x0A] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::BC,
                ..Instruction::default()
            };
            inst[0x0B] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::BC,
                ..Instruction::default()
            };
            inst[0x0C] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::C,
                ..Instruction::default()
            };
            inst[0x0D] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::C,
                ..Instruction::default()
            };
            inst[0x0E] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_D8,
                reg_1: RT::C,
                ..Instruction::default()
            };
        }

        //0x1X
        {
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
            inst[0x13] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::DE,
                ..Instruction::default()
            };
            inst[0x14] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::D,
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
            inst[0x18] = Instruction {
                in_type: IT::Jr,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0x19] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::HL,
                reg_2: RT::DE,
                ..Instruction::default()
            };
            inst[0x1A] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::DE,
                ..Instruction::default()
            };
            inst[0x1B] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::DE,
                ..Instruction::default()
            };
            inst[0x1C] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::E,
                ..Instruction::default()
            };
            inst[0x1D] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::E,
                ..Instruction::default()
            };
            inst[0x1E] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_D8,
                reg_1: RT::E,
                ..Instruction::default()
            };
        }
        //0x2X
        {
            inst[0x20] = Instruction {
                in_type: IT::Jr,
                mode: AM::D8,
                condition: CT::NZ,
                ..Instruction::default()
            };
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
            inst[0x23] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::HL,
                ..Instruction::default()
            };
            inst[0x24] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::H,
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
            inst[0x28] = Instruction {
                in_type: IT::Jr,
                mode: AM::D8,
                condition: CT::Z,
                ..Instruction::default()
            };
            inst[0x29] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::HL,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x2A] = Instruction {
                in_type: IT::Ld,
                mode: AM::HLI_R,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x2B] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::HL,
                ..Instruction::default()
            };
            inst[0x2C] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::L,
                ..Instruction::default()
            };
            inst[0x2D] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::L,
                ..Instruction::default()
            };
            inst[0x2E] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_D8,
                reg_1: RT::L,
                ..Instruction::default()
            };
        }
        //0x3X
        {
            inst[0x30] = Instruction {
                in_type: IT::Jr,
                mode: AM::D8,
                condition: CT::NC,
                ..Instruction::default()
            };
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
            inst[0x33] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::SP,
                ..Instruction::default()
            };
            inst[0x34] = Instruction {
                in_type: IT::Inc,
                mode: AM::MR,
                reg_1: RT::HL,
                ..Instruction::default()
            };
            inst[0x35] = Instruction {
                in_type: IT::Dec,
                mode: AM::MR,
                reg_1: RT::HL,
                ..Instruction::default()
            };
            inst[0x36] = Instruction {
                in_type: IT::Ld,
                mode: AM::MR_D8,
                reg_1: RT::HL,
                ..Instruction::default()
            };
            inst[0x38] = Instruction {
                in_type: IT::Jr,
                mode: AM::D8,
                condition: CT::C,
                ..Instruction::default()
            };
            inst[0x39] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::HL,
                reg_2: RT::SP,
                ..Instruction::default()
            };
            inst[0x3A] = Instruction {
                in_type: IT::Ld,
                mode: AM::HLD_R,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x3B] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::SP,
                ..Instruction::default()
            };
            inst[0x3C] = Instruction {
                in_type: IT::Inc,
                mode: AM::R,
                reg_1: RT::A,
                ..Instruction::default()
            };
            inst[0x3D] = Instruction {
                in_type: IT::Dec,
                mode: AM::R,
                reg_1: RT::A,
                ..Instruction::default()
            };
            inst[0x3E] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_D8,
                reg_1: RT::A,
                ..Instruction::default()
            };
        }
        //0x4X
        {
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
        }
        //0x5X
        {
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
        }
        //0x6X
        {
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
        }
        //0x7X
        {
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
        }
        //0x8X
        {
            inst[0x80] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0x81] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0x82] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0x83] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0x84] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0x85] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0x86] = Instruction {
                in_type: IT::Add,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x87] = Instruction {
                in_type: IT::Add,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
            inst[0x88] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0x89] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0x8A] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0x8B] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0x8C] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0x8D] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0x8E] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x8F] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
        }
        //0x9X
        {
            inst[0x90] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0x91] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0x92] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0x93] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0x94] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0x95] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0x96] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x97] = Instruction {
                in_type: IT::Sub,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
            inst[0x98] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0x99] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0x9A] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0x9B] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0x9C] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0x9D] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0x9E] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0x9F] = Instruction {
                in_type: IT::Sbc,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
        }
        //0xAX
        {
            inst[0xA0] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0xA1] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0xA2] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0xA3] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0xA4] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0xA5] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0xA6] = Instruction {
                in_type: IT::And,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0xA7] = Instruction {
                in_type: IT::And,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
            inst[0xA8] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0xA9] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0xAA] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0xAB] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0xAC] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0xAD] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0xAE] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0xAF] = Instruction {
                in_type: IT::Xor,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
        }
        //0xBX
        {
            inst[0xB0] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0xB1] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0xB2] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0xB3] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0xB4] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0xB5] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0xB6] = Instruction {
                in_type: IT::Or,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0xB7] = Instruction {
                in_type: IT::Or,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
            inst[0xB8] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::B,
                ..Instruction::default()
            };
            inst[0xB9] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::C,
                ..Instruction::default()
            };
            inst[0xBA] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::D,
                ..Instruction::default()
            };
            inst[0xBB] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::E,
                ..Instruction::default()
            };
            inst[0xBC] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::H,
                ..Instruction::default()
            };
            inst[0xBD] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::L,
                ..Instruction::default()
            };
            inst[0xBE] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_MR,
                reg_1: RT::A,
                reg_2: RT::HL,
                ..Instruction::default()
            };
            inst[0xBF] = Instruction {
                in_type: IT::Cp,
                mode: AM::R_R,
                reg_1: RT::A,
                reg_2: RT::A,
                ..Instruction::default()
            };
        }     
        //0xCX
        {
            inst[0xC0] = Instruction {
                in_type: IT::Ret,
                mode: AM::Imp,
                condition: CT::NZ,
                ..Instruction::default()
            };
            inst[0xC1] = Instruction {
                in_type: IT::Pop,
                mode: AM::R,
                reg_1: RT::BC,
                ..Instruction::default()
            };
            inst[0xC2] = Instruction {
                in_type: IT::Jp,
                mode: AM::D16,
                condition: CT::NZ,
                ..Instruction::default()
            };
            inst[0xC3] = Instruction {
                in_type: IT::Jp,
                mode: AM::D16,
                ..Instruction::default()
            };
            inst[0xC4] = Instruction {
                in_type: IT::Call,
                mode: AM::D16,
                condition: CT::NZ,
                ..Instruction::default()
            };
            inst[0xC5] = Instruction {
                in_type: IT::Push,
                mode: AM::R,
                reg_1: RT::BC,
                ..Instruction::default()
            };
            inst[0xC6] = Instruction {
                in_type: IT::Add,
                mode: AM::R_A8,
                reg_1: RT::A,
                ..Instruction::default()
            };
            inst[0xC7] = Instruction {
                in_type: IT::Rst,
                param: 0x00,
                ..Instruction::default()
            };
            inst[0xC8] = Instruction {
                in_type: IT::Ret,
                mode: AM::Imp,
                condition: CT::Z,
                ..Instruction::default()
            };
            inst[0xC9] = Instruction {
                in_type: IT::Ret,
                ..Instruction::default()
            };
            inst[0xCA] = Instruction {
                in_type: IT::Jp,
                mode: AM::D16,
                condition: CT::Z,
                ..Instruction::default()
            };
            inst[0xCB] = Instruction {
                in_type: IT::Cb,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xCC] = Instruction {
                in_type: IT::Call,
                mode: AM::D16,
                condition: CT::Z,
                ..Instruction::default()
            };
            inst[0xCD] = Instruction {
                in_type: IT::Call,
                mode: AM::D16,
                ..Instruction::default()
            };
            inst[0xCE] = Instruction {
                in_type: IT::Adc,
                mode: AM::R_D8,
                reg_1: RT::A,
                ..Instruction::default()
            };
            inst[0xCF] = Instruction {
                in_type: IT::Rst,
                param: 0x08,
                ..Instruction::default()
            };
        }
        //0xD0
        {
            inst[0xD0] = Instruction {
                in_type: IT::Ret,
                mode: AM::Imp,
                condition: CT::NC,
                ..Instruction::default()
            };
            inst[0xD1] = Instruction {
                in_type: IT::Pop,
                mode: AM::R,
                reg_1: RT::DE,
                ..Instruction::default()
            };
            inst[0xD2] = Instruction {
                in_type: IT::Jp,
                mode: AM::D16,
                condition: CT::NC,
                ..Instruction::default()
            };
            inst[0xD5] = Instruction {
                in_type: IT::Push,
                mode: AM::R,
                reg_1: RT::DE,
                ..Instruction::default()
            };
            inst[0xF6] = Instruction {
                in_type: IT::Sub,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xD7] = Instruction {
                in_type: IT::Rst,
                param: 0x10,
                ..Instruction::default()
            };
            inst[0xD8] = Instruction {
                in_type: IT::Ret,
                mode: AM::Imp,
                condition: CT::C,
                ..Instruction::default()
            };
            inst[0xD9] = Instruction {
                in_type: IT::Reti,
                ..Instruction::default()
            };
            inst[0xDC] = Instruction {
                in_type: IT::Call,
                mode: AM::D16,
                condition: CT::C,
                ..Instruction::default()
            };
            inst[0xDA] = Instruction {
                in_type: IT::Jp,
                mode: AM::D16,
                condition: CT::C,
                ..Instruction::default()
            };
            inst[0xDF] = Instruction {
                in_type: IT::Rst,
                param: 0x18,
                ..Instruction::default()
            };
        }
        //0xEX
        {
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
            inst[0xE6] = Instruction {
                in_type: IT::And,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xE7] = Instruction {
                in_type: IT::Rst,
                param: 0x20,
                ..Instruction::default()
            };
            inst[0xE8] = Instruction {
                in_type: IT::Add,
                mode: AM::R_D8,
                reg_1: RT::SP,
                ..Instruction::default()
            };
            inst[0xE9] = Instruction {
                in_type: IT::Jp,
                mode: AM::MR,
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
            inst[0xEE] = Instruction {
                in_type: IT::Xor,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xEF] = Instruction {
                in_type: IT::Rst,
                param: 0x28,
                ..Instruction::default()
            };
        }
        //0xFX
        {
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
            inst[0xF6] = Instruction {
                in_type: IT::Or,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xF7] = Instruction {
                in_type: IT::Rst,
                param: 0x30,
                ..Instruction::default()
            };
            inst[0xFA] = Instruction {
                in_type: IT::Ld,
                mode: AM::R_A16,
                reg_1: RT::A,
                ..Instruction::default()
            };
            inst[0xFE] = Instruction {
                in_type: IT::Cp,
                mode: AM::D8,
                ..Instruction::default()
            };
            inst[0xFF] = Instruction {
                in_type: IT::Rst,
                param: 0x38,
                ..Instruction::default()
            };
        }
        return inst;
    }
}

impl RegisterType {
    pub fn is_16bit(&self) -> bool {
        use RegisterType as RT;
        matches!(self, RT::AF | RT::BC | RT::DE | RT::HL | RT::SP | RT::PC)
    }
}

impl From<u8> for Instruction { // For CB instructions
    fn from(code: u8) -> Self {
        Instruction::INSTRUCTIONS[code as usize]
    }
}

impl From<u8> for RegisterType {
    fn from(value: u8) -> Self {
        use RegisterType as RT;
        match value {
            0 => RT::B,
            1 => RT::C,
            2 => RT::D,
            3 => RT::E,
            4 => RT::H,
            5 => RT::L,
            6 => RT::HL,
            7 => RT::A,
            _ => RT::None
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::default()
    }
}