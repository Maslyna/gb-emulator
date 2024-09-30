mod instruction;
mod regs;

use crate::{emu::Emu, memory::Bus};
use instruction::*;
use regs::*;

use AddressMode as AM;
use ConditionType as CT;
use RegisterType as RT;

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

    _interrupt_master_enabled: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: Instruction::default(),
            is_halted: false,
            _stepping: false,
            _interrupt_master_enabled: false,
        }
    }

    pub fn with_pc(pc: u16) -> Self {
        let mut cpu = Cpu::new();
        cpu.regs.pc = pc;

        cpu
    }

    pub fn step(&mut self, emu: &mut Emu, bus: &mut Bus) -> i32 {
        if self.is_halted {
            panic!("self EXEC FAILED");
        }

        let mut cycles = 0;
        self.fetch_instruction(bus);
        cycles += self.fetch_data(bus);
        cycles += self.execute(bus, emu);

        cycles
    }

    fn _process() {}

    pub fn execute(&mut self, bus: &mut Bus, emu: &mut Emu) -> i32 {
        let tick = emu.ticks;
        debug!(
            "{tick:08} - PC: {:04X} T:{:?}\tOP: ({:02X} {:02X} {:02X})\n\t\
                A: {:02X} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X} SP: {:04X}",
            self.regs.pc,
            self.cur_inst.in_type,
            self.cur_opcode,
            bus.read(self.regs.pc + 1),
            bus.read(self.regs.pc + 2),
            self.regs.a,
            self.regs.b,
            self.regs.c,
            self.regs.d,
            self.regs.e,
            self.regs.h,
            self.regs.l,
            self.regs.sp
        );
        debug!(
            "\tFLAGS: Z-{} C-{} H-{} N-{}",
            self.regs.flag_z() as i8,
            self.regs.flag_n() as i8,
            self.regs.flag_h() as i8,
            self.regs.flag_c() as i8
        );

        instruction::process_instruction(self, bus)
    }

    pub fn fetch_instruction(&mut self, bus: &Bus) {
        self.cur_opcode = bus.read(self.regs.pc);
        self.cur_inst = Instruction::from(self.cur_opcode);
        self.regs.pc += 1;
    }

    pub fn fetch_data(&mut self, bus: &Bus) -> i32 {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        let mut emu_cycles = 0;

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
                emu_cycles += 1;
                self.regs.pc += 1;
            }
            AM::A16Reg | AM::D16Reg => {
                let lo = bus.read(self.regs.pc);
                emu_cycles += 1;
                let hi = bus.read(self.regs.pc + 1);
                emu_cycles += 1;

                self.mem_dest = bytes_to_word!(lo, hi);
                self.dest_is_mem = true;
                self.fetched_data = self.read_reg(self.cur_inst.r2);
            }
            AM::D16 | AM::RegD16 => {
                let lo = bus.read(self.regs.pc);
                emu_cycles += 1;
                let hi = bus.read(self.regs.pc + 1);
                emu_cycles += 1;

                self.fetched_data = bytes_to_word!(lo, hi);
                self.regs.pc += 2;
            }
            AM::MemD8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                emu_cycles += 1;
                self.regs.pc += 1;
                self.mem_dest = self.read_reg(self.cur_inst.r1);
                self.dest_is_mem = true;
            }
            AM::Mem => {
                self.mem_dest = self.read_reg(self.cur_inst.r1);
                self.dest_is_mem = true;
                self.fetched_data = bus.read(self.read_reg(self.cur_inst.r1)) as u16;
                emu_cycles += 1;
            }
            AM::RegA16 => {
                let lo = bus.read(self.regs.pc);
                emu_cycles += 1;
                let hi = bus.read(self.regs.pc + 1);
                emu_cycles += 1;

                let addr = bytes_to_word!(lo, hi);

                self.regs.pc += 2;
                self.fetched_data = bus.read(addr) as u16;
                emu_cycles += 1;
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
                emu_cycles += 1;
            }
            AM::RegHLI => {
                self.fetched_data = bus.read(self.read_reg(self.cur_inst.r2)) as u16;
                emu_cycles += 1;
                self.set_reg(RT::HL, self.read_reg(RT::HL) + 1)
            }
            AM::RegHLD => {
                self.fetched_data = bus.read(self.read_reg(self.cur_inst.r2)) as u16;
                emu_cycles += 1;
                self.set_reg(RT::HL, self.read_reg(RT::HL) - 1)
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
                emu_cycles += 1;
                self.regs.pc += 1;
            }
            AM::A8Reg => {
                self.mem_dest = bus.read(self.regs.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                emu_cycles += 1;
                self.regs.pc += 1;
            }
            AM::HLRegsSP => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                emu_cycles += 1;
                self.regs.pc += 1;
            }
            AM::D8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                emu_cycles += 1;
                self.regs.pc += 1;
            } //_ => panic!("Unknown adressing mode: {:?}", self.cur_inst.mode),
        };

        return emu_cycles;
    }

    fn stack_push(&mut self, data: u8, bus: &mut Bus) {
        self.regs.sp -= 1;
        bus.write(self.regs.sp, data);
    }

    fn stack_push16(&mut self, data: u16, bus: &mut Bus) {
        self.stack_push(((data >> 8) & 0xFF) as u8, bus);
        self.stack_push((data & 0xFF) as u8, bus);
    }

    fn stack_pop(&mut self, bus: &Bus) -> u8 {
        let res = bus.read(self.regs.sp);
        self.regs.sp += 1;

        res
    }

    fn _stack_pop16(&mut self, bus: &mut Bus) -> u16 {
        let hi = self.stack_pop(bus);
        let lo = self.stack_pop(bus);

        bytes_to_word!(hi, lo)
    }

    fn read_reg(&self, reg_type: RegisterType) -> u16 {
        return match reg_type {
            RT::None => 0,
            RT::A => self.regs.a as u16,
            RT::F => self.regs.f as u16,
            RT::B => self.regs.b as u16,
            RT::C => self.regs.c as u16,
            RT::D => self.regs.d as u16,
            RT::E => self.regs.e as u16,
            RT::H => self.regs.h as u16,
            RT::L => self.regs.l as u16,
            RT::AF => reverse_u16!((self.regs.a as u16) << 8 | self.regs.f as u16),
            RT::BC => reverse_u16!((self.regs.b as u16) << 8 | self.regs.c as u16),
            RT::DE => reverse_u16!((self.regs.d as u16) << 8 | self.regs.e as u16),
            RT::HL => reverse_u16!((self.regs.h as u16) << 8 | self.regs.l as u16),
            RT::SP => self.regs.pc,
            RT::PC => self.regs.sp,
        };
    }

    fn read_reg8(&self, reg_type: RegisterType, bus: &mut Bus) -> u8 {
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
            _ => panic!("INVALID REG8: {reg_type:?}"),
        }
    }

    fn set_reg(&mut self, reg_type: RT, value: u16) {
        match reg_type {
            RT::None => panic!("Invalid register type!"),
            RT::A => self.regs.a = (value & 0xFF) as u8,
            RT::F => self.regs.f = (value & 0xFF) as u8,
            RT::B => self.regs.b = (value & 0xFF) as u8,
            RT::C => self.regs.c = (value & 0xFF) as u8,
            RT::D => self.regs.d = (value & 0xFF) as u8,
            RT::E => self.regs.e = (value & 0xFF) as u8,
            RT::H => self.regs.h = (value & 0xFF) as u8,
            RT::L => self.regs.l = (value & 0xFF) as u8,
            RT::AF => {
                let reversed = reverse_u16!(value);
                self.regs.a = (reversed & 0xFF) as u8;
                self.regs.f = (reversed >> 8) as u8;
            }
            RT::BC => {
                let reversed = reverse_u16!(value);
                self.regs.b = (reversed & 0xFF) as u8;
                self.regs.c = (reversed >> 8) as u8;
            }
            RT::DE => {
                let reversed = reverse_u16!(value);
                self.regs.d = (reversed & 0xFF) as u8;
                self.regs.e = (reversed >> 8) as u8;
            }
            RT::HL => {
                let reversed = reverse_u16!(value);
                self.regs.h = (reversed & 0xFF) as u8;
                self.regs.l = (reversed >> 8) as u8;
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
            _ => panic!("SET REG 8: INVALID REGISTER {reg_type:?}"),
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
