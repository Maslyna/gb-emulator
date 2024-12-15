mod instruction;
mod regs;

#[allow(unused_imports)]
use crate::common::*;
use crate::cpu::instruction::{
    AddressMode as AM, ConditionType as CT, Instruction, RegisterType as RT,
};
use crate::cpu::regs::Registers;
use crate::{memory::interrupts::handle, memory::Bus};
use std::fmt::Write;

const DEBUG: bool = true;

#[repr(u8)]
#[derive(Debug)]
pub enum InterruptAction {
    None,
    Enable,
    Disable,
}

#[derive(Debug)]
pub struct Cpu {
    pub regs: Registers,

    // current fetch
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,
    pub cur_inst: Instruction,

    pub is_halted: bool,
    _stepping: bool,

    pub interrupt_master_enabled: bool,
    pub enabling_ime: bool,
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: Instruction::default(),
            is_halted: false,
            _stepping: false,
            interrupt_master_enabled: true,
            enabling_ime: true,
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        if self.regs.pc == 518 {
            print!("");
        }

        if !self.is_halted {
            self.fetch_instruction(bus);

            bus.cycle(1);
            self.fetch_data(bus);
            

            if DEBUG {
                let instruction_view = instruction_to_str(self, bus);
                let debug_data = format!(
                    "{:08} - PC: {:04X} T: {}\tOP: ({:02X} {:02X} {:02X}) A: {:02X} FLAGS: {}{}{}{} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X} SP: {:04X}\n",
                    bus.timer.ticks,
                    self.regs.pc,
                    instruction_view,
                    self.cur_opcode,
                    bus.read(self.regs.pc + 1),
                    bus.read(self.regs.pc + 2),
                    self.regs.a,
                    if self.regs.flag_z() {'Z'} else {'-'},
                    if self.regs.flag_n() {'N'} else {'-'},
                    if self.regs.flag_h() {'H'} else {'-'},
                    if self.regs.flag_c() {'C'} else {'-'},
                    self.regs.b,
                    self.regs.c,
                    self.regs.d,
                    self.regs.e,
                    self.regs.h,
                    self.regs.l,
                    self.regs.sp).to_uppercase();
                print!("{}", debug_data);
                debug_write(&debug_data);
            }

            instruction::execute(self, bus);
        } else {
            bus.cycle(1);

            if bus.interrupts.flags != 0 {
                self.is_halted = false;
            }
        }

        if self.interrupt_master_enabled {
            handle(self, bus);
            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.interrupt_master_enabled = true;
        }
    }

    pub fn fetch_instruction(&mut self, bus: &Bus) {
        self.cur_opcode = bus.read(self.regs.pc);
        self.cur_inst = Instruction::from(self.cur_opcode);
        self.regs.pc = self.regs.pc.wrapping_add(1);
    }

    pub fn fetch_data(&mut self, bus: &mut Bus) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        match self.cur_inst.mode {
            AM::Imp => {}
            AM::Reg => {
                self.fetched_data = self.read_reg(self.cur_inst.r1);
            }
            AM::RegReg => {
                self.fetched_data = self.read_reg(self.cur_inst.r2);
            }
            AM::RegD8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                bus.cycle(1);
                self.regs.pc += 1;
            }
            AM::D16 | AM::RegD16 => {
                let lo = bus.read(self.regs.pc);
                bus.cycle(1);
                let hi = bus.read(self.regs.pc + 1);
                bus.cycle(1);

                self.fetched_data = bytes_to_word!(lo, hi);
                self.regs.pc += 2;
            }
            AM::MemReg => {
                self.fetched_data = self.read_reg(self.cur_inst.r2);
                self.mem_dest = self.read_reg(self.cur_inst.r1);
                self.dest_is_mem = true;

                if self.cur_inst.r1 == RT::C {
                    self.mem_dest |= 0xFF00;
                }
            }
            AM::RegMem => {
                let mut address = self.read_reg(self.cur_inst.r2);

                if self.cur_inst.r2 == RT::C {
                    address |= 0xFF00;
                }

                self.fetched_data = bus.read(address) as u16;
                bus.cycle(1);
            }
            AM::RegHLI => {
                let address = self.read_reg(self.cur_inst.r2);
                self.fetched_data = bus.read(address) as u16;
                bus.cycle(1);
                let data = self.read_reg(RT::HL).wrapping_add(1);
                self.set_reg(RT::HL, data);
            }
            AM::RegHLD => {
                let address = self.read_reg(self.cur_inst.r2);
                self.fetched_data = bus.read(address) as u16;
                bus.cycle(1);
                let data = self.read_reg(RT::HL).wrapping_sub(1);
                self.set_reg(RT::HL, data);
            }
            AM::HLIReg => {
                self.fetched_data = self.read_reg(self.cur_inst.r2);
                self.mem_dest = self.read_reg(self.cur_inst.r1);

                self.dest_is_mem = true;
                self.set_reg(RT::HL, self.read_reg(RT::HL) + 1);
            }
            AM::HLDReg => {
                self.fetched_data = self.read_reg(self.cur_inst.r2);
                self.mem_dest = self.read_reg(self.cur_inst.r1);

                self.dest_is_mem = true;
                self.set_reg(RT::HL, self.read_reg(RT::HL) - 1);
            }
            AM::RegA8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                bus.cycle(1);
                self.regs.pc += 1;
            }
            AM::A8Reg => {
                self.mem_dest = bus.read(self.regs.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                bus.cycle(1);
                self.regs.pc += 1;
            }
            AM::HLRegSPReg => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                bus.cycle(1);
                self.regs.pc += 1;
            }
            AM::D8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                bus.cycle(1);
                self.regs.pc += 1;
            }
            AM::A16Reg | AM::D16Reg => {
                let lo = bus.read(self.regs.pc);
                bus.cycle(1);
                let hi = bus.read(self.regs.pc + 1);
                bus.cycle(1);

                self.mem_dest = bytes_to_word!(lo, hi);
                self.dest_is_mem = true;

                self.regs.pc += 2;
                self.fetched_data = self.read_reg(self.cur_inst.r2);
            }
            AM::MemD8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                bus.cycle(1);
                self.regs.pc += 1;
                self.mem_dest = self.read_reg(self.cur_inst.r1);
                self.dest_is_mem = true;
            }
            AM::Mem => {
                self.mem_dest = self.read_reg(self.cur_inst.r1);
                self.dest_is_mem = true;
                let reg_1 = self.read_reg(self.cur_inst.r1);

                self.fetched_data = bus.read(reg_1) as u16;
                bus.cycle(1);
            }
            AM::RegA16 => {
                let lo = bus.read(self.regs.pc);
                bus.cycle(1);
                let hi = bus.read(self.regs.pc + 1);
                bus.cycle(1);

                let addr = bytes_to_word!(lo, hi);

                self.regs.pc += 2;
                self.fetched_data = bus.read(addr) as u16;
                bus.cycle(1);
            } //_ => panic!("Unknown adressing mode: {:?}", self.cur_inst.mode),
        };
    }

    pub fn stack_push(&mut self, data: u8, bus: &mut Bus) {
        self.regs.sp = self.regs.sp.wrapping_sub(1);
        bus.write(self.regs.sp, data);
    }

    pub fn stack_push16(&mut self, data: u16, bus: &mut Bus) {
        self.stack_push(((data >> 8) & 0xFF) as u8, bus);
        self.stack_push((data & 0xFF) as u8, bus);
    }

    fn stack_pop(&mut self, bus: &Bus) -> u8 {
        let res = bus.read(self.regs.sp);
        self.regs.sp = self.regs.sp.wrapping_add(1);

        res
    }

    fn _stack_pop16(&mut self, bus: &mut Bus) -> u16 {
        let hi = self.stack_pop(bus);
        let lo = self.stack_pop(bus);

        ((hi as u16) << 8) | lo as u16
    }

    fn read_reg(&self, reg_type: RT) -> u16 {
        match reg_type {
            RT::None => 0,
            RT::A => self.regs.a as u16,
            RT::F => self.regs.f as u16,
            RT::B => self.regs.b as u16,
            RT::C => self.regs.c as u16,
            RT::D => self.regs.d as u16,
            RT::E => self.regs.e as u16,
            RT::H => self.regs.h as u16,
            RT::L => self.regs.l as u16,
            RT::AF => ((self.regs.a as u16) << 8) | (self.regs.f as u16),
            RT::BC => ((self.regs.b as u16) << 8) | (self.regs.c as u16),
            RT::DE => ((self.regs.d as u16) << 8) | (self.regs.e as u16),
            RT::HL => ((self.regs.h as u16) << 8) | (self.regs.l as u16),
            // RT::AF => (self.regs.a as u16) << 8 | (self.regs.f & 0xF0) as u16,
            // RT::BC => (self.regs.b as u16) << 8 | self.regs.c as u16,
            // RT::DE => (self.regs.d as u16) << 8 | self.regs.e as u16,
            // RT::HL => (self.regs.h as u16) << 8 | self.regs.l as u16,
            RT::SP => self.regs.pc,
            RT::PC => self.regs.sp,
        }
    }

    fn read_reg8(&self, reg_type: RT, bus: &mut Bus) -> u8 {
        match reg_type {
            RT::None => 0,
            RT::A => self.regs.a,
            RT::F => self.regs.f,
            RT::B => self.regs.b,
            RT::C => self.regs.c,
            RT::D => self.regs.d,
            RT::E => self.regs.e,
            RT::H => self.regs.h,
            RT::L => self.regs.l,
            RT::HL => bus.read(self.read_reg(reg_type)),
            _ => panic!("INVALID REG8: {:?}", reg_type),
        }
    }

    fn set_reg(&mut self, reg_type: RT, value: u16) {
        match reg_type {
            RT::None => panic!("Invalid register type!"),
            RT::A => self.regs.a = value as u8,
            RT::F => self.regs.f = value as u8,
            RT::B => self.regs.b = value as u8,
            RT::C => self.regs.c = value as u8,
            RT::D => self.regs.d = value as u8,
            RT::E => self.regs.e = value as u8,
            RT::H => self.regs.h = value as u8,
            RT::L => self.regs.l = value as u8,
            RT::AF => {
                // let reversed = reverse(value);
                // self.regs.a = (reversed & 0xFF) as u8;
                // self.regs.f = (reversed >> 8) as u8;

                // self.regs.a = (value >> 8) as u8;
                // self.regs.f = (value & 0x00F0) as u8;

                self.regs.a = ((value & 0xFF00) >> 8) as u8;
                self.regs.f = value as u8;
            }
            RT::BC => {
                // let reversed = reverse(value);
                // self.regs.b = (reversed & 0xFF) as u8;
                // self.regs.c = (reversed >> 8) as u8;

                // self.regs.b = (value >> 8) as u8;
                // self.regs.c = (value & 0x00FF) as u8;

                self.regs.b = ((value & 0xFF00) >> 8) as u8;
                self.regs.c = value as u8;
            }
            RT::DE => {
                // let reversed = reverse(value);
                // self.regs.d = (reversed & 0xFF) as u8;
                // self.regs.e = (reversed >> 8) as u8;

                // self.regs.d = (value >> 8) as u8;
                // self.regs.e = (value & 0x00FF) as u8;

                self.regs.d = ((value & 0xFF00) >> 8) as u8;
                self.regs.e = value as u8;
            }
            RT::HL => {
                // let reversed = reverse(value);
                // self.regs.h = (reversed & 0xFF) as u8;
                // self.regs.l = (reversed >> 8) as u8;

                // self.regs.h = (value >> 8) as u8;
                // self.regs.l = (value & 0x00FF) as u8;

                self.regs.h = ((value & 0xFF00) >> 8) as u8;
                self.regs.l = value as u8;
            }
            RT::SP => self.regs.sp = value,
            RT::PC => self.regs.pc = value,
        };
    }

    fn set_reg8(&mut self, reg_type: RT, value: u8, bus: &mut Bus) {
        match reg_type {
            RT::A => self.regs.a = value, // value & 0xFF,
            RT::F => self.regs.f = value, // value & 0xFF,
            RT::B => self.regs.b = value, // value & 0xFF,
            RT::C => self.regs.c = value, // value & 0xFF,
            RT::D => self.regs.d = value, // value & 0xFF,
            RT::E => self.regs.e = value, // value & 0xFF,
            RT::H => self.regs.h = value, // value & 0xFF,
            RT::L => self.regs.l = value, // value & 0xFF,
            RT::HL => {
                bus.write(self.read_reg(RT::HL), value);
            }
            _ => panic!("SET REG 8: INVALID REGISTER {:?}", reg_type),
        };
    }

    #[inline(always)]
    fn check_cond(&self) -> bool {
        match self.cur_inst.condition {
            CT::None => true,
            CT::NZ => !self.regs.flag_z(),
            CT::Z => self.regs.flag_z(),
            CT::NC => !self.regs.flag_c(),
            CT::C => self.regs.flag_c(),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

fn instruction_to_str(cpu: &Cpu, bus: &Bus) -> String {
    let inst = cpu.cur_inst;
    let fetched_data = cpu.fetched_data;
    let mut result = String::with_capacity(16);

    write!(&mut result, "{:?}", cpu.cur_inst.in_type).unwrap();

    match inst.mode {
        AM::Imp => {}
        AM::RegD16 | AM::RegA16 => {
            write!(&mut result, " {:?}, ${:04X}", inst.r1, fetched_data).unwrap();
        }
        AM::Reg => {
            write!(&mut result, " {:?}", inst.r1).unwrap();
        }
        AM::RegReg => {
            write!(&mut result, " {:?}, {:?}", inst.r1, inst.r2).unwrap();
        }
        AM::MemReg => {
            write!(&mut result, " ({:?}), {:?}", inst.r1, inst.r2).unwrap();
        }
        AM::RegMem => {
            write!(&mut result, " {:?}, ({:?})", inst.r1, inst.r2).unwrap();
        }
        AM::RegD8 | AM::RegA8 => {
            write!(&mut result, " {:?}, ${:02X}", inst.r1, fetched_data as u8).unwrap();
        }
        AM::RegHLI => {
            write!(&mut result, " {:?}, ({:?}+)", inst.r1, inst.r2).unwrap();
        }
        AM::RegHLD => {
            write!(&mut result, " {:?}, ({:?}-)", inst.r1, inst.r2).unwrap();
        }
        AM::HLIReg => {
            write!(&mut result, " ({:?}+), {:?}", inst.r1, inst.r2).unwrap();
        }
        AM::HLDReg => {
            write!(&mut result, " ({:?}-), {:?}", inst.r1, inst.r2).unwrap();
        }
        AM::A8Reg => {
            write!(
                &mut result,
                " ${:02X}, {:?}",
                bus.read(fetched_data),
                inst.r2
            )
            .unwrap();
        }
        AM::HLRegSPReg => {
            write!(&mut result, " ({:?}), SP+{:?}", inst.r1, fetched_data as u8).unwrap();
        }
        AM::D8 => {
            write!(&mut result, " ${:02X}", fetched_data as u8).unwrap();
        }
        AM::D16 => {
            write!(&mut result, " ${:04X}", fetched_data).unwrap();
        }
        AM::MemD8 => {
            write!(&mut result, " ({:?}),${:02X}", inst.r1, fetched_data as u8).unwrap();
        }
        AM::A16Reg => {
            write!(&mut result, " (${:04X}), {:?}", fetched_data, inst.r2).unwrap();
        }
        _ => {
            result.push_str("INVALID MODE");
        }
    };

    while result.len() < 16 {
        result.push(' ');
    }

    result
}
