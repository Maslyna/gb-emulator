mod instruction;

use crate::{bus::Bus, emu::Emu};
use instruction::*;

use std::cell::RefCell;
use std::rc::Rc;

type RcMut<T> = Rc<RefCell<T>>;
type OpRcMut<T> = Option<RcMut<T>>;

#[derive(Debug, Default)]
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

#[derive(Debug)]
pub struct Cpu {
    pub emu: OpRcMut<Emu>,
    pub bus: OpRcMut<Bus>,

    pub regs: Registers,
    pub ie_reg: u8,

    // current fetch
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,
    cur_inst: Instruction,

    halted: bool,
    _stepping: bool,

    int_master_enabled: bool,
}

impl Cpu {
    fn get_emu(&self) -> &RcMut<Emu> {
        self.emu.as_ref().expect("NO EMU PROVIDED")
    }

    fn get_bus(&self) -> &RcMut<Bus> {
        self.bus.as_ref().expect("NO BUS PROVIDED")
    }

    fn emu_cycles(&mut self, cycles: i32) {
        self.get_emu().borrow_mut().cycle(cycles);
    }

    fn bus_read(&self, address: u16) -> u8 {
        self.get_bus().borrow().read(address)
    }

    fn _bus_read16(&self, address: u16) -> u16 {
        self.get_bus().borrow().read16(address)
    }

    fn bus_write(&self, address: u16, value: u8) {
        self.get_bus().borrow_mut().write(address, value);
    }

    fn bus_write16(&self, address: u16, value: u16) {
        self.get_bus().borrow_mut().write16(address, value);
    }

    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            ie_reg: 0,
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: Instruction::default(),
            halted: false,
            _stepping: false,
            int_master_enabled: false,
            emu: None,
            bus: None,
        }
    }

    pub fn with_pc(pc: u16) -> Self {
        let mut cpu = Cpu::new();
        cpu.regs.pc = pc;

        cpu
    }

    pub fn step(&mut self) -> bool {
        if !self.halted {
            self.fetch_instruction();
            self.fetch_data();
            self.execute();
        }

        return true;
    }

    fn execute(&mut self) {
        let tick = self.get_emu().borrow().ticks;
        debug!(
            "{tick:08} - PC: {:04X} T:{:?}\tOP: ({:02X} {:02X} {:02X})\n\t\
                A: {:02X} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X} SP: {:04X}",
            self.regs.pc,
            self.cur_inst.in_type,
            self.cur_opcode,
            self.bus_read(self.regs.pc + 1),
            self.bus_read(self.regs.pc + 2),
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
            self.flag_z() as i8,
            self.flag_n() as i8,
            self.flag_h() as i8,
            self.flag_c() as i8
        );
        use InstructionType as IT;
        match self.cur_inst.in_type {
            IT::None => panic!("INVALID INSTRUCTION: {:?}", self.cur_inst),
            IT::Nop => self.nop_in(),
            IT::Ld => self.ld_in(),
            IT::Inc => self.inc_in(),
            IT::Dec => self.dec_in(),
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
            IT::Jr => self.jr_in(),
            IT::Pop => self.pop_in(),
            IT::Jp => self.jp_in(),
            IT::Push => self.push_in(),
            IT::Ret => self.ret_in(),
            IT::Reti => self.reti_in(),
            IT::Cb => self.cb_in(),
            IT::Call => self.call_in(),
            IT::Ldh => self.ldh_in(),
            IT::Jphl => todo!(),
            IT::Di => self.di_in(),
            IT::Ei => todo!(),
            IT::Rst => self.rst_in(),
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
        };
    }

    fn fetch_instruction(&mut self) {
        self.cur_opcode = self.bus_read(self.regs.pc);
        self.cur_inst = Instruction::from(self.cur_opcode);
        self.regs.pc += 1;
    }

    fn fetch_data(&mut self) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        use AddressMode as AM;
        use RegisterType as RT;
        match self.cur_inst.mode {
            AM::Imp => {}
            AM::R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_1);
            }
            AM::R_R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
            }
            AM::R_D8 => {
                self.fetched_data = self.bus_read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            }
            AM::A16_R | AM::D16_R => {
                let lo = self.bus_read(self.regs.pc);
                self.emu_cycles(1);
                let hi = self.bus_read(self.regs.pc + 1);
                self.emu_cycles(1);

                self.mem_dest = combine_bytes!(lo, hi);
                self.dest_is_mem = true;
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
            }
            AM::D16 | AM::R_D16 => {
                let lo = self.bus_read(self.regs.pc);
                self.emu_cycles(1);
                let hi = self.bus_read(self.regs.pc + 1);
                self.emu_cycles(1);

                self.fetched_data = combine_bytes!(lo, hi);
                self.regs.pc += 2;
            }
            AM::MR_D8 => {
                self.fetched_data = self.bus_read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);
                self.dest_is_mem = true;
            }
            AM::MR => {
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);
                self.dest_is_mem = true;
                self.fetched_data = self.bus_read(self.read_reg(self.cur_inst.reg_1)) as u16;
                self.emu_cycles(1);
            }
            AM::R_A16 => {
                let lo = self.bus_read(self.regs.pc);
                self.emu_cycles(1);
                let hi = self.bus_read(self.regs.pc + 1);
                self.emu_cycles(1);

                let addr = combine_bytes!(lo, hi);

                self.regs.pc += 2;
                self.fetched_data = self.bus_read(addr) as u16;
                self.emu_cycles(1);
            }
            AM::MR_R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);
                self.dest_is_mem = true;

                if self.cur_inst.reg_1 == RT::C {
                    self.mem_dest |= 0xFF00;
                }
            }
            AM::R_MR => {
                let mut address = self.read_reg(self.cur_inst.reg_2);

                if self.cur_inst.reg_2 == RT::C {
                    address |= 0xFF00;
                }

                self.fetched_data = self.bus_read(address) as u16;
                self.emu_cycles(1);
            }
            AM::R_HLI => {
                self.fetched_data = self.bus_read(self.read_reg(self.cur_inst.reg_2)) as u16;
                self.emu_cycles(1);
                self.set_reg(RT::HL, self.read_reg(RT::HL) + 1)
            }
            AM::R_HLD => {
                self.fetched_data = self.bus_read(self.read_reg(self.cur_inst.reg_2)) as u16;
                self.emu_cycles(1);
                self.set_reg(RT::HL, self.read_reg(RT::HL) - 1)
            }
            AM::HLI_R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);

                self.dest_is_mem = true;
                self.set_reg(RT::HL, self.read_reg(RT::HL) + 1);
            }
            AM::HLD_R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);

                self.dest_is_mem = true;
                self.set_reg(RT::HL, self.read_reg(RT::HL) - 1);
            }
            AM::R_A8 => {
                self.fetched_data = self.bus_read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            }
            AM::A8_R => {
                self.mem_dest = self.bus_read(self.regs.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                self.emu_cycles(1);
                self.regs.pc += 1;
            }
            AM::HL_SPR => {
                self.fetched_data = self.bus_read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            }
            AM::D8 => {
                self.fetched_data = self.bus_read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            } //_ => panic!("Unknown adressing mode: {:?}", self.cur_inst.mode),
        };
    }

    fn stack_push(&mut self, data: u8) {
        self.regs.sp -= 1;
        self.bus_write(self.regs.sp, data);
    }

    fn stack_push16(&mut self, data: u16) {
        self.stack_push(((data >> 8) & 0xFF) as u8);
        self.stack_push((data & 0xFF) as u8);
    }

    fn stack_pop(&mut self) -> u8 {
        let res = self.bus_read(self.regs.sp);
        self.regs.sp += 1;

        res
    }

    fn _stack_pop16(&mut self) -> u16 {
        let hi = self.stack_pop();
        let lo = self.stack_pop();

        combine_bytes!(hi, lo)
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

    fn read_reg8(&self, reg_type: RegisterType) -> u8 {
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
            RT::HL => self.bus_read(self.read_reg(reg_type)),
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

    fn set_reg8(&mut self, reg_type: RegisterType, value: u8) {
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
                self.bus_write(self.read_reg(RT::HL), value);
            }
            _ => panic!("SET REG 8: INVALID REGISTER {reg_type:?}"),
        };
    }

    fn _process() {}

    fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z >= 0 {
            set_bit!(self.regs.f, 7, z != 0);
        }
        if n >= 0 {
            set_bit!(self.regs.f, 6, n != 0);
        }
        if h >= 0 {
            set_bit!(self.regs.f, 5, h != 0);
        }
        if c >= 0 {
            set_bit!(self.regs.f, 4, c != 0);
        }
    }

    #[inline(always)]
    fn flag_z(&self) -> bool {
        bit!(self.regs.f, 7)
    }

    #[inline(always)]
    fn flag_n(&self) -> bool {
        bit!(self.regs.f, 6)
    }

    #[inline(always)]
    fn flag_h(&self) -> bool {
        bit!(self.regs.f, 5)
    }

    #[inline(always)]
    fn flag_c(&self) -> bool {
        bit!(self.regs.f, 4)
    }

    #[inline(always)]
    fn check_cond(&self) -> bool {
        use ConditionType::*;
        match self.cur_inst.condition {
            None => true,
            NZ => !self.flag_z(),
            Z => self.flag_z(),
            NC => !self.flag_c(),
            C => self.flag_c(),
        }
    }

    #[inline(always)]
    fn nop_in(&self) {}

    fn ld_in(&mut self) {
        use AddressMode as AM;

        if self.dest_is_mem {
            //e.g.: LD (BC) A
            if self.cur_inst.reg_2.is_16bit() {
                self.emu_cycles(1);
                self.bus_write16(self.mem_dest, self.fetched_data);
            } else {
                self.bus_write(self.mem_dest, self.fetched_data as u8);
            }

            self.emu_cycles(1);

            return;
        }

        if self.cur_inst.mode == AM::HL_SPR {
            let hflag = ((self.read_reg(self.cur_inst.reg_2) & 0xF) + (self.fetched_data & 0xF)
                >= 0x10) as i8;
            let cflag = ((self.read_reg(self.cur_inst.reg_2) & 0xFF) + (self.fetched_data & 0xFF)
                >= 0x100) as i8;

            self.set_flags(0, 0, hflag, cflag);
            self.set_reg(
                self.cur_inst.reg_1,
                self.read_reg(self.cur_inst.reg_2) + self.fetched_data,
            );

            return;
        }

        self.set_reg(self.cur_inst.reg_1, self.fetched_data);
    }

    fn ldh_in(&mut self) {
        use RegisterType as RT;
        match self.cur_inst.reg_1 {
            RT::A => {
                self.set_reg(
                    self.cur_inst.reg_1,
                    self.bus_read(0xFF00 | self.fetched_data) as u16,
                );
            }
            _ => {
                self.bus_write(0xFF00 | self.fetched_data, self.regs.a);
            }
        }
        self.emu_cycles(1);
    }

    fn goto(&mut self, address: u16, pushpc: bool) {
        if self.check_cond() {
            if pushpc {
                self.stack_push16(self.regs.pc);
                self.emu_cycles(2);
            }

            self.regs.pc = address;
            self.emu_cycles(1);
        }
    }

    #[inline(always)]
    fn jp_in(&mut self) {
        self.goto(self.fetched_data, false);
    }

    #[inline(always)]
    fn call_in(&mut self) {
        self.goto(self.fetched_data, true);
    }

    #[inline(always)]
    fn rst_in(&mut self) {
        self.goto(self.cur_inst.param as u16, true);
    }

    #[inline(always)]
    fn jr_in(&mut self) {
        let r: i8 = (self.fetched_data & 0xFF) as i8;
        let addr: u16 = (self.regs.pc as i16 + r as i16) as u16;
        self.goto(addr, false);
    }

    fn ret_in(&mut self) {
        use ConditionType as CT;

        if self.cur_inst.condition != CT::None {
            self.emu_cycles(1);
        }

        if self.check_cond() {
            let lo = self.stack_pop();
            self.emu_cycles(1);
            let hi = self.stack_pop();
            self.emu_cycles(1);

            let value = combine_bytes!(lo, hi);
            self.regs.pc = value;

            self.emu_cycles(1);
        }
    }

    #[inline(always)]
    fn reti_in(&mut self) {
        self.int_master_enabled = true;
        self.ret_in();
    }

    fn inc_in(&mut self) {
        use AddressMode as AM;
        use RegisterType as RT;

        let mut val = self.read_reg(self.cur_inst.reg_1) + 1;

        if self.cur_inst.reg_1.is_16bit() {
            self.emu_cycles(1);
        }

        if self.cur_inst.reg_1 == RT::HL && self.cur_inst.mode == AM::MR {
            let reg_hl = self.read_reg(RT::HL);
            val = (self.bus_read(reg_hl) + 1) as u16;
            val &= 0xFF;
            self.bus_write(reg_hl, val as u8);
        } else {
            self.set_reg(self.cur_inst.reg_1, val);
            val = self.read_reg(self.cur_inst.reg_1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags((val == 0) as i8, 0, ((val & 0xFF) == 0) as i8, -1);
    }

    fn dec_in(&mut self) {
        use AddressMode as AM;
        use RegisterType as RT;

        let mut val = self.read_reg(self.cur_inst.reg_1) - 1;

        if self.cur_inst.reg_1.is_16bit() {
            self.emu_cycles(1);
        }

        if self.cur_inst.reg_1 == RT::HL && self.cur_inst.mode == AM::MR {
            let reg_hl = self.read_reg(RT::HL);
            val = (self.bus_read(reg_hl) - 1) as u16;
            self.bus_write(reg_hl, val as u8);
        } else {
            self.set_reg(self.cur_inst.reg_1, val);
            val = self.read_reg(self.cur_inst.reg_1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1);
    }

    fn add_in(&mut self) {
        use RegisterType as RT;

        let reg_val: u16 = self.read_reg(self.cur_inst.reg_1);
        let is_16bit: bool = self.cur_inst.reg_1.is_16bit();
        let is_sp: bool = self.cur_inst.reg_1 == RT::SP;
        let val: u32 = if is_sp {
            (reg_val + self.fetched_data as i8 as u16) as u32
        } else {
            (reg_val + self.fetched_data) as u32
        };

        if is_16bit {
            self.emu_cycles(1);
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
        self.set_reg(self.cur_inst.reg_1, val as u16 & 0xFFFF);
        self.set_flags(z as i8, 0, h as i8, c as i8);
    }

    fn adc_in(&mut self) {
        let u = self.fetched_data;
        let a = self.regs.a as u16;
        let c = self.flag_c() as u16;

        self.regs.a = ((a + u + c) & 0xFF) as u8;

        self.set_flags(
            (self.regs.a == 0) as i8,
            0,
            (a & 0xF) as i8 + (u & 0xF) as i8 + (c > 0xF) as i8,
            (a + u + c > 0xFF) as i8,
        )
    }

    fn sub_in(&mut self) {
        let reg_val = self.read_reg(self.cur_inst.reg_1);
        let val = reg_val - self.fetched_data;

        let z: i32 = (val == 0) as i32;
        let h: i32 = (((reg_val & 0xF) as i32 - (self.fetched_data & 0xF) as i32) < 0) as i32;
        let c: i32 = ((reg_val as i32) - (self.fetched_data as i32) < 0) as i32;

        self.set_reg(self.cur_inst.reg_1, val);
        self.set_flags(z as i8, 1, h as i8, c as i8);
    }

    fn sbc_in(&mut self) {
        let flag_c = self.flag_c();
        let val = (self.fetched_data + (flag_c as u16)) as u8;
        let reg_val = self.read_reg(self.cur_inst.reg_1);

        let z: i32 = (reg_val - val as u16 == 0) as i32;
        let h: i32 = ((reg_val as i32 & 0xF) - (self.fetched_data as i32 & 0xF) - (flag_c as i32)
            < 0) as i32;
        let c: i32 = ((reg_val as i32) - (self.fetched_data as i32) - (flag_c as i32) < 0) as i32;

        self.set_reg(self.cur_inst.reg_1, reg_val - val as u16);
        self.set_flags(z as i8, 1, h as i8, c as i8);
    }

    fn and_in(&mut self) {
        self.regs.a &= self.fetched_data as u8;
        self.set_flags((self.regs.a == 0) as i8, 0, 1, 0);
    }

    fn or_in(&mut self) {
        self.regs.a |= self.fetched_data as u8;
        self.set_flags((self.regs.a == 0) as i8, 0, 0, 0);
    }

    fn cp_in(&mut self) {
        let z = ((self.regs.a as i32 - self.fetched_data as i32) == 0) as i8;
        let h = (((self.regs.a as i32 & 0x0F) - (self.fetched_data as i32 & 0x0F)) < 0) as i8;

        self.set_flags(z, 1, h, (z < 0) as i8);
    }

    fn cb_in(&mut self) {
        let operation = self.fetched_data;
        let reg = RegisterType::from(operation as u8 & 0b111);
        let mut reg_val = self.read_reg8(reg);
        let bit = (operation as u8 >> 3) & 0b111;
        let bit_op = (operation as u8 >> 6) & 0b11;

        self.emu_cycles(1);

        use RegisterType as RT;
        if reg == RT::HL {
            self.emu_cycles(2);
        }

        match bit_op {
            1 => {
                // BIT
                let flag_z = !(reg_val & (1 << bit));
                self.set_flags(flag_z as i8, 0, 1, -1);
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                self.set_reg8(reg, reg_val);
            }
            3 => {
                // SET
                reg_val |= !(1 << bit);
                self.set_reg8(reg, reg_val);
                return;
            }
            _ => {}
        };

        let flag_c = self.flag_c();

        match bit {
            0 => {
                // RLC
                let mut set_c = false;
                let mut result = reg_val << 1; // (reg_val << 1) & 0xFF;

                if (reg_val & (1 << 7)) != 0 {
                    result |= 1;
                    set_c = true;
                }

                self.set_reg8(reg, result);
                self.set_flags((result == 0) as i8, 0, 0, set_c as i8);
                return;
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= old << 7;

                self.set_reg8(reg, reg_val);
                self.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
                return;
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;

                self.set_reg8(reg, reg_val);
                self.set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
                return;
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= (flag_c as u8) << 7;

                self.set_reg8(reg, reg_val);
                self.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                self.set_reg8(reg, reg_val);
                self.set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
                return;
            }
            5 => {
                // SRA
                let u = (reg_val as i8 >> 1) as u8;

                self.set_reg8(reg, u);
                self.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
                return;
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);

                self.set_reg8(reg, reg_val);
                self.set_flags((reg_val == 0) as i8, 0, 0, 0);
                return;
            }
            7 => {
                // SRL
                let u = reg_val >> 1;

                self.set_reg8(reg, u);
                self.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
                return;
            }
            _ => panic!("INVALID CB INSTRUCTION: {operation:02X}"),
        };
    }

    fn xor_in(&mut self) {
        self.regs.a ^= (self.fetched_data & 0xFF) as u8;
        self.set_flags((self.regs.a == 0) as i8, 0, 0, 0);
    }

    fn pop_in(&mut self) {
        let lo = self.stack_pop();
        self.emu_cycles(1);
        let hi = self.stack_pop();
        self.emu_cycles(1);

        use RegisterType as RT;

        let val = combine_bytes!(lo, hi);
        let reg_1 = self.cur_inst.reg_1;

        self.set_reg(reg_1, val);

        if reg_1 == RT::AF {
            self.set_reg(reg_1, val & 0xFFF0)
        };
    }

    fn push_in(&mut self) {
        let hi = (self.read_reg(self.cur_inst.reg_1) >> 8) & 0xFF;
        self.emu_cycles(1);
        self.stack_push(hi as u8);

        let lo = self.read_reg(self.cur_inst.reg_2) & 0xFF;
        self.emu_cycles(1);
        self.stack_push(lo as u8);

        self.emu_cycles(1);
    }

    #[inline(always)]
    fn di_in(&mut self) {
        self.int_master_enabled = false;
    }
}

impl Registers {
    pub const fn new() -> Self {
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
