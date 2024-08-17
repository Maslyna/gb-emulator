use super::instructions::*;
use crate::{bus::Bus, emu::Emu};

use std::cell::RefCell;
use std::rc::Rc;

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
    pub emu: Option<Rc<RefCell<Emu>>>,
    pub bus: Option<Rc<RefCell<Bus>>>,

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
    fn emu_cycles(&mut self, cycles: i32) {
        self.emu
            .as_ref()
            .expect("NO EMU PROVIDED")
            .borrow_mut()
            .cycle(cycles);
    }

    fn bus_read(&self, address: u16) -> u8 {
        self.bus
            .as_ref()
            .expect("NO BUS PROVIDED")
            .borrow()
            .read(address)
    }

    fn _bus_read16(&self, address: u16) -> u16 {
        self.bus
            .as_ref()
            .expect("NO BUS PROVIDED")
            .borrow()
            .read16(address)
    }

    fn bus_write(&self, address: u16, value: u8) {
        self.bus
            .as_ref()
            .expect("NO BUS PROVIDED")
            .borrow_mut()
            .write(address, value);
    }

    fn bus_write16(&self, address: u16, value: u16) {
        self.bus
            .as_ref()
            .expect("NO BUS PROVIDED")
            .borrow_mut()
            .write16(address, value);
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
        debug!(
            "PC: {:04X} T:{:?}\tOP: ({:02X} {:02X} {:02X})\n\t\
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
        use InstructionType as IT;
        match self.cur_inst.in_type {
            IT::None => panic!("INVALID INSTRUCTION: {:?}", self.cur_inst),
            IT::Nop => self.nop(),
            IT::Ld => self.ld(),
            IT::Inc => self.inc(),
            IT::Dec => self.dec(),
            IT::Rlca => todo!(),
            IT::Add => todo!(),
            IT::Rrca => todo!(),
            IT::Stop => todo!(),
            IT::Rla => todo!(),
            IT::Rra => todo!(),
            IT::Daa => todo!(),
            IT::Cpl => todo!(),
            IT::Scf => todo!(),
            IT::Ccf => todo!(),
            IT::Halt => todo!(),
            IT::Adc => todo!(),
            IT::Sub => todo!(),
            IT::Sbc => todo!(),
            IT::And => todo!(),
            IT::Xor => self.xor(),
            IT::Or => todo!(),
            IT::Cp => todo!(),
            IT::Jr => self.jr(),
            IT::Pop => self.pop(),
            IT::Jp => self.jp(),
            IT::Push => self.push(),
            IT::Ret => self.ret(),
            IT::Reti => self.reti(),
            IT::Cb => todo!(),
            IT::Call => self.call(),
            IT::Ldh => self.ldh(),
            IT::Jphl => todo!(),
            IT::Di => self.di(),
            IT::Ei => todo!(),
            IT::Rst => self.rst(),
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

    fn flag_z(&self) -> bool {
        bit!(self.regs.f, 7)
    }

    fn _flag_n(&self) -> bool {
        bit!(self.regs.f, 6)
    }

    fn _flag_h(&self) -> bool {
        bit!(self.regs.f, 5)
    }

    fn flag_c(&self) -> bool {
        bit!(self.regs.f, 4)
    }

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

    fn nop(&self) {}

    fn ld(&mut self) {
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

    fn ldh(&mut self) {
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

    fn jp(&mut self) {
        self.goto(self.fetched_data, false);
    }

    fn call(&mut self) {
        self.goto(self.fetched_data, true);
    }

    fn rst(&mut self) {
        self.goto(self.cur_inst.param as u16, true);
    }

    fn jr(&mut self) {
        let r: i8 = (self.fetched_data & 0xFF) as i8;
        let addr: u16 = (self.regs.pc as i16 + r as i16) as u16;
        self.goto(addr, false);
    }

    fn ret(&mut self) {
        use ConditionType as CT;

        if self.cur_inst.condition != CT::None {
            self.emu_cycles(1);
        }

        if self.check_cond() {
            let lo = self.stack_pop();
            self.emu_cycles(1);
            let hi = self.stack_pop();
            self.emu_cycles(1);

            let v = combine_bytes!(lo, hi);
            self.regs.pc = v;

            self.emu_cycles(1);
        }
    }

    fn reti(&mut self) {
        self.int_master_enabled = true;
        self.ret();
    }

    fn inc(&mut self) {
        use AddressMode as AM;
        use RegisterType as RT;

        let mut val = self.read_reg(self.cur_inst.reg_1) + 1;

        if self.cur_inst.reg_1.is_16bit() {
            self.emu_cycles(1);
        }

        if self.cur_inst.reg_1 == RT::HL && self.cur_inst.mode == AM::MR {
            val = (self.bus_read(self.read_reg(RT::HL)) + 1) as u16;
            val &= 0xFF;
            self.bus_write(self.read_reg(RT::HL), val as u8);
        } else {
            self.set_reg(self.cur_inst.reg_1, val);
            val = self.read_reg(self.cur_inst.reg_1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags((val == 0) as i8, 0, ((val & 0xFF) == 0) as i8, -1);
    }

    fn dec(&mut self) {
        use AddressMode as AM;
        use RegisterType as RT;

        let mut val = self.read_reg(self.cur_inst.reg_1) - 1;

        if self.cur_inst.reg_1.is_16bit() {
            self.emu_cycles(1);
        }

        if self.cur_inst.reg_1 == RT::HL && self.cur_inst.mode == AM::MR {
            val = (self.bus_read(self.read_reg(RT::HL)) - 1) as u16;
            self.bus_write(self.read_reg(RT::HL), val as u8);
        } else {
            self.set_reg(self.cur_inst.reg_1, val);
            val = self.read_reg(self.cur_inst.reg_1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return;
        }

        self.set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1);
    }

    fn pop(&mut self) {
        let lo = self.stack_pop();
        self.emu_cycles(1);
        let hi = self.stack_pop();
        self.emu_cycles(1);

        let val = combine_bytes!(lo, hi);

        use RegisterType as RT;

        self.set_reg(self.cur_inst.reg_1, val);

        if self.cur_inst.reg_1 == RT::AF {
            self.set_reg(self.cur_inst.reg_1, val & 0xFFF0)
        };
    }

    fn push(&mut self) {
        let hi = (self.read_reg(self.cur_inst.reg_1) >> 8) & 0xFF;
        self.emu_cycles(1);
        self.stack_push(hi as u8);

        let lo = self.read_reg(self.cur_inst.reg_2) & 0xFF;
        self.emu_cycles(1);
        self.stack_push(lo as u8);

        self.emu_cycles(1);
    }

    fn di(&mut self) {
        self.int_master_enabled = false;
    }

    fn xor(&mut self) {
        self.regs.a ^= (self.fetched_data & 0xFF) as u8;
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
