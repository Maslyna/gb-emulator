use super::instructions::*;
use crate::bus::Bus;

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

pub struct Cpu {
    pub regs: Registers,

    // current fetch
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    cur_opcode: u8,
    cur_inst: Instruction,

    halted: bool,
    stepping: bool,

    int_master_enabled: bool,
}

impl Cpu {
    pub const fn new() -> Self {
        Self {
            regs: Registers::new(),
            fetched_data: 0,
            mem_dest: 0,
            dest_is_mem: false,
            cur_opcode: 0,
            cur_inst: Instruction::from_op_code(0),
            halted: false,
            stepping: false,
            int_master_enabled: false,
        }
    }

    pub const fn with_pc(pc: u16) -> Self {
        let mut cpu = Cpu::new();
        cpu.regs.pc = pc;

        cpu
    }

    pub fn step(&mut self, bus: &mut Bus) -> bool {
        if !self.halted {
            self.fetch_instruction(bus);
            self.fetch_data(bus);
            self.execute();
        }

        return true;
    }

    fn execute(&mut self) {
        println!(
            "PC: {:04X} T:{:?}\tOP: {:02X}\n\tA: {:02X} B: {:02X} C: {:02X}",
            self.regs.pc,
            self.cur_inst.in_type,
            self.cur_opcode,
            self.regs.a,
            self.regs.b,
            self.regs.c
        );
        use InstructionType as IT;
        match self.cur_inst.in_type {
            IT::None => panic!("INVALID INSTRUCTION"),
            IT::Nop => self.nop(),
            IT::Ld => self.ld(),
            IT::Inc => todo!(),
            IT::Dec => todo!(),
            IT::Rlca => todo!(),
            IT::Add => todo!(),
            IT::Rrca => todo!(),
            IT::Stop => todo!(),
            IT::Rla => todo!(),
            IT::Jr => todo!(),
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
            IT::Xor => todo!(),
            IT::Or => todo!(),
            IT::Cp => todo!(),
            IT::Pop => todo!(),
            IT::Jp => self.jp(),
            IT::Push => todo!(),
            IT::Ret => todo!(),
            IT::Cb => todo!(),
            IT::Call => todo!(),
            IT::Reti => todo!(),
            IT::Ldh => todo!(),
            IT::Jphl => todo!(),
            IT::Di => self.di(),
            IT::Ei => todo!(),
            IT::Rst => todo!(),
            IT::Err => todo!(),
            IT::Rlc => todo!(),
            IT::Rrc => todo!(),
            IT::RL => todo!(),
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

    fn fetch_instruction(&mut self, bus: &mut Bus) {
        self.cur_opcode = bus.read(self.regs.pc);
        self.cur_inst = Instruction::from_op_code(self.cur_opcode);
        self.regs.pc += 1;
    }

    fn fetch_data(&mut self, bus: &mut Bus) {
        self.mem_dest = 0;
        self.dest_is_mem = false;

        use AdressMode as AM;
        use RegisterType as RT;
        match self.cur_inst.mode {
            AM::Imp => {}
            AM::R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_1);
            }
            AM::R_D8 => {
                self.fetched_data = bus.read(self.regs.pc) as u16;
                self.emu_cycles(1);
                self.regs.pc += 1;
            }
            AM::D16 | AM::R_D16 => {
                let lo = bus.read(self.regs.pc);
                self.emu_cycles(1);
                let hi = bus.read(self.regs.pc + 1);
                self.emu_cycles(1);
                
                self.fetched_data = combine_bytes!(lo, hi);
                self.regs.pc += 2;
            }
            AM::MR_R => {
                self.fetched_data = self.read_reg(self.cur_inst.reg_2);
                self.mem_dest = self.read_reg(self.cur_inst.reg_1);
                self.dest_is_mem = false;

                if self.cur_inst.reg_1 == RT::C {
                    self.mem_dest |= 0xFF00;
                }
            }
            AM::R_MR => {
                let mut address = self.read_reg(self.cur_inst.reg_2);

                if self.cur_inst.reg_1 == RT::C {
                    address |= 0xFF00;
                }

                self.fetched_data = bus.read(address) as u16;
                self.emu_cycles(1);
            }
            _ => panic!("Unknown adressing mode: {:?}", self.cur_inst.mode),
        };
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
            RT::AF => reverse_u16!(self.regs.a as u16),
            RT::BC => reverse_u16!(self.regs.b as u16),
            RT::DE => reverse_u16!(self.regs.d as u16),
            RT::HL => reverse_u16!(self.regs.h as u16),
            RT::SP => self.regs.pc,
            RT::PC => self.regs.sp,
        };
    }

    fn process() {}

    fn emu_cycles(&self, _cycles: i32) {}

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

    fn flag_n(&self) -> bool {
        bit!(self.regs.f, 6)
    }

    fn flag_h(&self) -> bool {
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

    fn ld(&mut self) {}

    fn jp(&mut self) {
        if self.check_cond() {
            self.regs.pc = self.fetched_data;
            self.emu_cycles(1);
        }
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
