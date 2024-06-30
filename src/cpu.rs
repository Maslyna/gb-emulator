#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
use instructions::*;

pub struct CPU {
    regs: Registers,
    fetch_data: u16,
    mem_dest: u16,
    cur_opcode: u8,
    cur_in: Instruction,
    halted: bool,
    stepping: bool,
}

pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl CPU {

    pub fn step(&self) -> bool {
        if !self.halted {
            
        }

        return false;
    }
}

pub mod instructions {
    #[derive(Clone, Copy)]
    pub struct Instruction {
        in_type: InstructionType,
        mode: AdressMode,
        reg_1: RegisterType,
        reg_2: RegisterType,
        condition: ConditionType,
        param: u8,
    }

    lazy_static::lazy_static! {
        pub static ref INSTRUCTIONS: [Instruction; 0x100] = {
            let mut instructions: [Instruction; 0x100] = [Instruction {
                in_type: InstructionType::None,
                mode: AdressMode::Imp,
                reg_1: RegisterType::None,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            }; 0x100];

            instructions[0x00] = Instruction {
                in_type: InstructionType::NOP,
                mode: AdressMode::Imp,
                reg_1: RegisterType::None,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            };

            instructions[0x05] = Instruction {
                in_type: InstructionType::DEC,
                mode: AdressMode::R,
                reg_1: RegisterType::B,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            };

            instructions[0x0E] = Instruction {
                in_type: InstructionType::LD,
                mode: AdressMode::R_D8,
                reg_1: RegisterType::C,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            };

            instructions[0xAF] = Instruction {
                in_type: InstructionType::XOR,
                mode: AdressMode::R,
                reg_1: RegisterType::A,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            };

            instructions[0xC3] = Instruction {
                in_type: InstructionType::JP,
                mode: AdressMode::D16,
                reg_1: RegisterType::None,
                reg_2: RegisterType::None,
                condition: ConditionType::None,
                param: 0,
            };

            return instructions;
        };
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

    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    pub enum InstructionType {
        None,
        NOP,
        LD,
        INC,
        DEC,
        RLCA,
        ADD,
        RRCA,
        STOP,
        RLA,
        JR,
        RRA,
        DAA,
        CPL,
        SCF,
        CCF,
        HALT,
        ADC,
        SUB,
        SBC,
        AND,
        XOR,
        OR,
        CP,
        POP,
        JP,
        PUSH,
        RET,
        CB,
        CALL,
        RETI,
        LDH,
        JPHL,
        DI,
        EI,
        RST,
        ERR,
        //CB instructions...
        RLC,
        RRC,
        RL,
        RR,
        SLA,
        SRA,
        SWAP,
        SRL,
        BIT,
        RES,
        SET,
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
}
