#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct Instruction {
    pub in_type: InstructionType,
    pub mode: AdressMode,
    pub reg_1: RegisterType,
    pub reg_2: RegisterType,
    pub condition: ConditionType,
    pub param: u8,
}

impl Instruction {
    const INSTRUCTIONS: [Instruction; 0x100] = Instruction::init_instructions();

    pub fn from_op_code(code: u8) -> Instruction {
        return Instruction::INSTRUCTIONS[code as usize];
    }

    const fn init_instructions() -> [Instruction; 0x100] {
        let mut instructions: [Instruction; 0x100] = [Instruction::default(); 0x100];

        instructions[0x00] = Instruction {
            in_type: InstructionType::Nop,
            ..Instruction::default()
        };

        instructions[0x05] = Instruction {
            in_type: InstructionType::Dec,
            mode: AdressMode::R,
            reg_1: RegisterType::B,
            ..Instruction::default()
        };

        instructions[0x0E] = Instruction {
            in_type: InstructionType::Ld,
            mode: AdressMode::R_D8,
            reg_1: RegisterType::C,
            ..Instruction::default()
        };

        instructions[0xAF] = Instruction {
            in_type: InstructionType::Xor,
            mode: AdressMode::R,
            reg_1: RegisterType::A,
            ..Instruction::default()
        };

        instructions[0xC3] = Instruction {
            in_type: InstructionType::Jp,
            mode: AdressMode::D16,
            ..Instruction::default()
        };

        instructions[0xF3] = Instruction {
            in_type: InstructionType::Di,
            ..Instruction::default()
        };
        
        return instructions;
    }

    const fn default() -> Self {
        Self {
            in_type: InstructionType::None,
            mode: AdressMode::Imp,
            reg_1: RegisterType::None,
            reg_2: RegisterType::None,
            condition: ConditionType::None,
            param: 0,
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction::default()
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum AdressMode {
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}
