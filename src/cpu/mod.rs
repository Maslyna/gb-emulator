mod instructions;

use instructions::*;
use super::bus::Bus;

pub struct Cpu<'a> {
    bus: &'a Bus,

    regs: Registers,

    // current fetch
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,
    cur_inst: Instruction,

    halted: bool,
    stepping: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a Bus) -> Self {
        Self {
            bus,
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: Instruction::from_op_code(0),
            halted: false,
            stepping: false,
        }
    }

    pub fn step(&mut self) -> bool {
        if !self.halted {
            self.fetch_instruction();
            self.fetch_data();
            self.execute();
        }

        return false;
    }

    fn execute(&self) {
        println!("Not executing yet...");
    }

    fn fetch_instruction(&mut self) {
        self.cur_opcode = self.bus.read(self.regs.pc);
        self.cur_inst = Instruction::from_op_code(self.cur_opcode);
        self.regs.pc += 1;
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match self.cur_inst.mode {
            AdressMode::Imp => {},
            AdressMode::R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_1);
            },
            AdressMode::R_D8 => {
                self.fetched_data = self.bus.read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            },
            AdressMode::D16 => {
                let lo = self.bus.read(self.regs.pc) as u16;
                self.emu_cycles(1);
                let hi = self.bus.read(self.regs.pc + 1) as u16;
                self.emu_cycles(1);
                self.fetched_data = lo | (hi << 8);
                self.regs.pc += 2;
            },
            _ => panic!("Unknown adressing mode: {:?}", self.cur_inst.mode),
        }
    }

    fn read_reg(&self, reg_type: RegisterType) -> u16 {
        return match reg_type {
            RegisterType::None => 0,
            RegisterType::A => self.regs.a as u16,
            RegisterType::F => self.regs.f as u16,
            RegisterType::B => self.regs.b as u16,
            RegisterType::C => self.regs.c as u16,
            RegisterType::D => self.regs.d as u16,
            RegisterType::E => self.regs.e as u16,
            RegisterType::H => self.regs.h as u16,
            RegisterType::L => self.regs.l as u16,
            RegisterType::AF => Cpu::reverse(self.regs.a as u16),
            RegisterType::BC => Cpu::reverse(self.regs.b as u16),
            RegisterType::DE => Cpu::reverse(self.regs.d as u16),
            RegisterType::HL => Cpu::reverse(self.regs.h as u16),
            RegisterType::SP => self.regs.pc,
            RegisterType::PC => self.regs.sp,
        };
    }

    fn reverse(n: u16) -> u16 {
        ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
    } 

    fn emu_cycles(&self, _cycles: i32) {
        
    }

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

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }
}