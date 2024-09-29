mod instruction;
mod regs;

use crate::{memory::Bus, emu::Emu};
use instruction::*;
use regs::*;

#[derive(Debug)]
pub struct Cpu {
    pub regs: Registers,

    // current fetch
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,
    cur_inst: Instruction,

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
        use InstructionType as IT;
        
        return match self.cur_inst.in_type {
            IT::None => panic!("INVALID INSTRUCTION: {:?}", self.cur_inst),
            IT::Nop => self.nop_in(),
            IT::Ld => self.ld_in(bus),
            IT::Inc => self.inc_in(bus),
            IT::Dec => self.dec_in(bus),
            IT::Rlca => todo!(),
            IT::Add => self.add_in(),
            IT::Rrca => todo!(),
            IT::Stop => todo!(),
            IT::Rla => todo!(),
            IT::Rra => todo!(),
            IT::Daa => todo!(),
            IT::Cpl => todo!(),
            IT::Scf => todo!(),
            IT::Ccf => todo!(),
            IT::Halt => todo!(),
            IT::Adc => self.adc_in(),
            IT::Sub => self.sub_in(),
            IT::Sbc => self.sbc_in(),
            IT::And => self.and_in(),
            IT::Xor => self.xor_in(),
            IT::Or => self.or_in(),
            IT::Cp => self.cp_in(),
            IT::Jr => self.jr_in(bus),
            IT::Pop => self.pop_in(bus),
            IT::Jp => self.jp_in(bus),
            IT::Push => self.push_in(bus),
            IT::Ret => self.ret_in(bus),
            IT::Reti => self.reti_in(bus),
            IT::Cb => self.cb_in(bus),
            IT::Call => self.call_in(bus),
            IT::Ldh => self.ldh_in(bus),
            IT::Jphl => todo!(),
            IT::Di => self.di_in(),
            IT::Ei => todo!(),
            IT::Rst => self.rst_in(bus),
            IT::Err => todo!(),
            IT::Rlc => todo!(),
            IT::Rrc => todo!(),
            IT::Rl => todo!(),
            IT::Rr => todo!(),
            IT::Sla => todo!(),
            IT::Sra => todo!(),
            IT::Swap => todo!(),
            IT::Srl => todo!(),
            IT::Bit => todo!(),
            IT::Res => todo!(),
            IT::Set => todo!(),
        }
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

        use AddressMode as AM;
        use RegisterType as RT;
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
        use RegisterType as RT;
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
        use RegisterType as RT;
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

    fn set_reg(&mut self, reg_type: RegisterType, value: u16) {
        use RegisterType as RT;
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

    fn set_reg8(&mut self, reg_type: RegisterType, value: u8, bus: &mut Bus) {
        use RegisterType as RT;

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

    fn _process() {}

    #[inline(always)]
    fn check_cond(&self) -> bool {
        use ConditionType::*;
        match self.cur_inst.condition {
            None => true,
            NZ => !self.regs.flag_z(),
            Z => self.regs.flag_z(),
            NC => !self.regs.flag_c(),
            C => self.regs.flag_c(),
        }
    }

    #[inline(always)]
    fn nop_in(&self) -> i32 {0}

    fn ld_in(&mut self, bus: &mut Bus) -> i32 {
        use AddressMode as AM;
        let mut emu_cycles = 0;
        if self.dest_is_mem {
            //e.g.: LD (BC) A
            if self.cur_inst.r2.is_16bit() {
                emu_cycles += 1;
                bus.write16(self.mem_dest, self.fetched_data);
            } else {
                bus.write(self.mem_dest, self.fetched_data as u8);
            }

            emu_cycles += 1;

            return emu_cycles;
        }

        if self.cur_inst.mode == AM::HLRegsSP {
            let hflag =
                ((self.read_reg(self.cur_inst.r2) & 0xF) + (self.fetched_data & 0xF) >= 0x10) as i8;
            let cflag = ((self.read_reg(self.cur_inst.r2) & 0xFF) + (self.fetched_data & 0xFF)
                >= 0x100) as i8;

            self.regs.set_flags(0, 0, hflag, cflag);
            self.set_reg(
                self.cur_inst.r1,
                self.read_reg(self.cur_inst.r2) + self.fetched_data,
            );

            return emu_cycles;
        }

        self.set_reg(self.cur_inst.r1, self.fetched_data);
        return emu_cycles;
    }

    fn ldh_in(&mut self, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;

        use RegisterType as RT;
        match self.cur_inst.r1 {
            RT::A => {
                self.set_reg(
                    self.cur_inst.r1,
                    bus.read(0xFF00 | self.fetched_data) as u16,
                );
            }
            _ => {
                bus.write(0xFF00 | self.fetched_data, self.regs.a);
            }
        }
        emu_cycles += 1;
        return emu_cycles;
    }

    fn goto(&mut self, address: u16, pushpc: bool, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;
        if self.check_cond() {
            if pushpc {
                self.stack_push16(self.regs.pc, bus);
                emu_cycles += 2;
            }

            self.regs.pc = address;
            emu_cycles += 1;
        }
        return emu_cycles;
    }

    #[inline(always)]
    fn jp_in(&mut self, bus: &mut Bus) -> i32 {
        self.goto(self.fetched_data, false, bus)
    }

    #[inline(always)]
    fn call_in(&mut self, bus: &mut Bus) -> i32 {
        self.goto(self.fetched_data, true, bus)
    }

    #[inline(always)]
    fn rst_in(&mut self, bus: &mut Bus) -> i32 {
        self.goto(self.cur_inst.param as u16, true, bus)
    }

    #[inline(always)]
    fn jr_in(&mut self, bus: &mut Bus) -> i32 {
        let r: i8 = (self.fetched_data & 0xFF) as i8;
        let addr: u16 = (self.regs.pc as i16 + r as i16) as u16;

        self.goto(addr, false, bus)
    }

    fn ret_in(&mut self, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;

        use ConditionType as CT;

        if self.cur_inst.condition != CT::None {
            emu_cycles += 1;
        }

        if self.check_cond() {
            let lo = self.stack_pop(bus);
            emu_cycles += 1;
            let hi = self.stack_pop(bus);
            emu_cycles += 1;

            let value = bytes_to_word!(lo, hi);
            self.regs.pc = value;

            emu_cycles += 1;
        }
        return emu_cycles;
    }

    #[inline(always)]
    fn reti_in(&mut self, bus: &mut Bus) -> i32 {
        self._interrupt_master_enabled = true;
        self.ret_in(bus)
    }

    fn inc_in(&mut self, bus: &mut Bus) -> i32 {
        use AddressMode as AM;
        use RegisterType as RT;

        let mut emu_cycles = 0;
        let mut val = self.read_reg(self.cur_inst.r1) + 1;

        if self.cur_inst.r1.is_16bit() {
            emu_cycles += 1;
        }

        if self.cur_inst.r1 == RT::HL && self.cur_inst.mode == AM::Mem {
            let reg_hl = self.read_reg(RT::HL);
            val = (bus.read(reg_hl) + 1) as u16;
            val &= 0xFF;
            bus.write(reg_hl, val as u8);
        } else {
            self.set_reg(self.cur_inst.r1, val);
            val = self.read_reg(self.cur_inst.r1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return emu_cycles;
        }

        self.regs
            .set_flags((val == 0) as i8, 0, ((val & 0xFF) == 0) as i8, -1);

        emu_cycles
    }

    fn dec_in(&mut self, bus: &mut Bus) -> i32 {
        use AddressMode as AM;
        use RegisterType as RT;
        let mut emu_cycles = 0;
        let mut val = self.read_reg(self.cur_inst.r1) - 1;

        if self.cur_inst.r1.is_16bit() {
            emu_cycles += 1;
        }

        if self.cur_inst.r1 == RT::HL && self.cur_inst.mode == AM::Mem {
            let reg_hl = self.read_reg(RT::HL);
            val = (bus.read(reg_hl) - 1) as u16;
            bus.write(reg_hl, val as u8);
        } else {
            self.set_reg(self.cur_inst.r1, val);
            val = self.read_reg(self.cur_inst.r1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return emu_cycles;
        }

        self.regs
            .set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1);

        emu_cycles
    }

    fn add_in(&mut self) -> i32 {
        use RegisterType as RT;

        let mut emu_cycles = 0;

        let reg_val: u16 = self.read_reg(self.cur_inst.r1);
        let is_16bit: bool = self.cur_inst.r1.is_16bit();
        let is_sp: bool = self.cur_inst.r1 == RT::SP;
        let val: u32 = if is_sp {
            (reg_val + self.fetched_data as i8 as u16) as u32
        } else {
            (reg_val + self.fetched_data) as u32
        };

        if is_16bit {
            emu_cycles += 1;
        }

        let (z, h, c) = if is_sp {
            (
                0,
                ((reg_val & 0xF) + (self.fetched_data & 0xF) >= 0x10) as i32,
                ((reg_val & 0xFF) as i32 + (self.fetched_data & 0xFF) as i32 > 0x100) as i32,
            )
        } else if is_16bit {
            let n: u32 = (reg_val as u32) + (self.fetched_data as u32);
            (
                -1,
                ((reg_val & 0xFFF) + (self.fetched_data & 0xFFF) >= 0x1000) as i32,
                (n >= 0x10000) as i32,
            )
        } else {
            (
                ((val & 0xFF) == 0) as i32,
                ((reg_val & 0xF) + (self.fetched_data & 0xF) >= 0x10) as i32,
                ((reg_val & 0xFF) as i32 + (self.fetched_data & 0xFF) as i32 >= 0x100) as i32,
            )
        };

        #[allow(clippy::identity_op)]
        self.set_reg(self.cur_inst.r1, val as u16 & 0xFFFF);
        self.regs.set_flags(z as i8, 0, h as i8, c as i8);

        emu_cycles
    }

    fn adc_in(&mut self) -> i32 {
        let u = self.fetched_data;
        let a = self.regs.a as u16;
        let c = self.regs.flag_c() as u16;

        self.regs.a = ((a + u + c) & 0xFF) as u8;

        self.regs.set_flags(
            (self.regs.a == 0) as i8,
            0,
            (a & 0xF) as i8 + (u & 0xF) as i8 + (c > 0xF) as i8,
            (a + u + c > 0xFF) as i8,
        );

        0
    }

    fn sub_in(&mut self) -> i32 {
        let reg_val = self.read_reg(self.cur_inst.r1);
        let val = reg_val.wrapping_sub(self.fetched_data);

        let z: i32 = (val == 0) as i32;
        let h: i32 = (((reg_val & 0xF) as i32 - (self.fetched_data & 0xF) as i32) < 0) as i32;
        let c: i32 = ((reg_val as i32) - (self.fetched_data as i32) < 0) as i32;

        self.set_reg(self.cur_inst.r1, val);
        self.regs.set_flags(z as i8, 1, h as i8, c as i8);

        0
    }

    fn sbc_in(&mut self) -> i32 {
        let flag_c = self.regs.flag_c();
        let val = (self.fetched_data + (flag_c as u16)) as u8;
        let reg_val = self.read_reg(self.cur_inst.r1);

        let z: i32 = (reg_val - val as u16 == 0) as i32;
        let h: i32 = ((reg_val as i32 & 0xF) - (self.fetched_data as i32 & 0xF) - (flag_c as i32)
            < 0) as i32;
        let c: i32 = ((reg_val as i32) - (self.fetched_data as i32) - (flag_c as i32) < 0) as i32;

        self.set_reg(self.cur_inst.r1, reg_val - val as u16);
        self.regs.set_flags(z as i8, 1, h as i8, c as i8);

        0
    }

    fn and_in(&mut self) -> i32 {
        self.regs.a &= self.fetched_data as u8;
        self.regs.set_flags((self.regs.a == 0) as i8, 0, 1, 0);

        0
    }

    fn or_in(&mut self) -> i32 {
        self.regs.a |= self.fetched_data as u8;
        self.regs.set_flags((self.regs.a == 0) as i8, 0, 0, 0);

        0
    }

    fn cp_in(&mut self) -> i32 {
        let z = ((self.regs.a as i32 - self.fetched_data as i32) == 0) as i8;
        let h = (((self.regs.a as i32 & 0x0F) - (self.fetched_data as i32 & 0x0F)) < 0) as i8;

        self.regs.set_flags(z, 1, h, (z < 0) as i8);

        0
    }

    fn cb_in(&mut self, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;
        let operation = self.fetched_data;
        let reg = RegisterType::from(operation as u8 & 0b111);
        let mut reg_val = self.read_reg8(reg, bus);
        let bit = (operation as u8 >> 3) & 0b111;
        let bit_op = (operation as u8 >> 6) & 0b11;

        emu_cycles += 1;

        use RegisterType as RT;
        if reg == RT::HL {
            emu_cycles += 2;
        }

        match bit_op {
            1 => {
                // BIT
                let flag_z = !(reg_val & (1 << bit));
                self.regs.set_flags(flag_z as i8, 0, 1, -1);
                return emu_cycles;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                self.set_reg8(reg, reg_val, bus);
            }
            3 => {
                // SET
                reg_val |= !(1 << bit);
                self.set_reg8(reg, reg_val, bus);
                return emu_cycles;
            }
            _ => {}
        };

        let flag_c = self.regs.flag_c();

        match bit {
            0 => {
                // RLC
                let mut set_c = false;
                let mut result = reg_val << 1; // (reg_val << 1) & 0xFF;

                if (reg_val & (1 << 7)) != 0 {
                    result |= 1;
                    set_c = true;
                }

                self.set_reg8(reg, result, bus);
                self.regs.set_flags((result == 0) as i8, 0, 0, set_c as i8);
                return emu_cycles;
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= old << 7;

                self.set_reg8(reg, reg_val, bus);
                self.regs.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
                return emu_cycles;
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;

                self.set_reg8(reg, reg_val, bus);
                self.regs
                    .set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
                return emu_cycles;
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= (flag_c as u8) << 7;

                self.set_reg8(reg, reg_val, bus);
                self.regs.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                self.set_reg8(reg, reg_val, bus);
                self.regs
                    .set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
                return emu_cycles;
            }
            5 => {
                // SRA
                let u = (reg_val as i8 >> 1) as u8;

                self.set_reg8(reg, u, bus);
                self.regs.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
                return emu_cycles;
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);

                self.set_reg8(reg, reg_val, bus);
                self.regs.set_flags((reg_val == 0) as i8, 0, 0, 0);
                return emu_cycles;
            }
            7 => {
                // SRL
                let u = reg_val >> 1;

                self.set_reg8(reg, u, bus);
                self.regs.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
                return emu_cycles;
            }
            _ => panic!("INVALID CB INSTRUCTION: {operation:02X}"),
        };

        emu_cycles
    }

    fn xor_in(&mut self) -> i32 {
        self.regs.a ^= (self.fetched_data & 0xFF) as u8;
        self.regs.set_flags((self.regs.a == 0) as i8, 0, 0, 0);

        0
    }

    fn pop_in(&mut self, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;
        let lo = self.stack_pop(bus);
        emu_cycles += 1;
        let hi = self.stack_pop(bus);
        emu_cycles += 1;

        use RegisterType as RT;

        let val = bytes_to_word!(lo, hi);
        let reg_1 = self.cur_inst.r1;

        self.set_reg(reg_1, val);

        if reg_1 == RT::AF {
            self.set_reg(reg_1, val & 0xFFF0)
        };

        emu_cycles
    }

    fn push_in(&mut self, bus: &mut Bus) -> i32 {
        let mut emu_cycles = 0;
        let hi = (self.read_reg(self.cur_inst.r1) >> 8) & 0xFF;
        emu_cycles += 1;
        self.stack_push(hi as u8, bus);

        let lo = self.read_reg(self.cur_inst.r2) & 0xFF;
        emu_cycles += 1;
        self.stack_push(lo as u8, bus);

        emu_cycles += 1;

        emu_cycles
    }

    #[inline(always)]
    fn di_in(&mut self) -> i32 {
        self._interrupt_master_enabled = false;
        
        0
    }
}
