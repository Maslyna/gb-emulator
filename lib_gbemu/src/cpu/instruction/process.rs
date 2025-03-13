use self::instruction::{AddressMode as AM, ConditionType as CT, InstructionType as IT};
use self::regs::CpuFlag as Flag;
use crate::cpu::*;
use crate::memory::*;

impl Cpu {
    pub fn execute(&mut self, bus: &mut Bus) {
        match self.cur_inst.in_type {
            IT::None => panic!(
                "INVALID INSTRUCTION: {:?}, {}",
                self.cur_inst, self.cur_opcode
            ),
            IT::Nop => self.nop_in(),
            IT::Ld => self.ld_in(bus),
            IT::Inc => self.inc_in(bus),
            IT::Dec => self.dec_in(bus),
            IT::Add => self.add_in(bus),
            IT::Sub => self.sub_in(),
            IT::Sbc => self.sbc_in(),
            IT::Rlca => self.rlca_in(),
            IT::Rrca => self.rrca_in(),
            IT::Stop => self.stop_in(),
            IT::Rla => self.rla_in(),
            IT::Rra => self.rra_in(),
            IT::Daa => self.daa_in(),
            IT::Cpl => self.cpl_in(),
            IT::Scf => self.scf_in(),
            IT::Ccf => self.ccf_in(),
            IT::Halt => self.halt_in(),
            IT::Adc => self.adc_in(),
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
            IT::Ei => self.ei_in(),
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

    #[inline(always)]
    fn nop_in(&self) {}

    fn ld_in(&mut self, bus: &mut Bus) {
        if self.dest_is_mem {
            //e.g.: LD (BC) A
            if self.cur_inst.r2.is_16bit() {
                bus.cycle(1);
                bus.write16(self.mem_dest, self.fetched_data);
            } else {
                bus.write(self.mem_dest, self.fetched_data as u8);
            }

            bus.cycle(1);

            return;
        }

        if self.cur_inst.mode == AM::HLRegSPReg {
            let reg_val = self.read_reg(self.cur_inst.r2);
            let fetched_data = self.fetched_data;
            let hflag = ((reg_val & 0xF) + (fetched_data & 0xF)) >= 0x10;
            let cflag = ((reg_val & 0xFF) + (fetched_data & 0xFF)) >= 0x100;

            self.regs.set_flags(false, false, hflag, cflag);
            self.set_reg(
                self.cur_inst.r1,
                self.read_reg(self.cur_inst.r2)
                    .wrapping_add_signed(fetched_data as i8 as i16),
            );

            return;
        }

        self.set_reg(self.cur_inst.r1, self.fetched_data);
    }

    fn ldh_in(&mut self, bus: &mut Bus) {
        match self.cur_inst.r1 {
            RT::A => {
                let data = bus.read(0xFF00 | self.fetched_data) as u16;
                self.set_reg(self.cur_inst.r1, data);
            }
            _ => {
                bus.write(self.mem_dest, self.regs.a);
            }
        }

        bus.cycle(1);
    }

    fn goto_in(&mut self, bus: &mut Bus, address: u16, pushpc: bool) {
        if self.check_cond() {
            if pushpc {
                bus.cycle(2);
                self.stack_push16(self.regs.pc, bus);
            }

            self.regs.pc = address;
            bus.cycle(1);
        }
    }

    fn jp_in(&mut self, bus: &mut Bus) {
        self.goto_in(bus, self.fetched_data, false);
    }

    fn jr_in(&mut self, bus: &mut Bus) {
        let rel = (self.fetched_data & 0xFF) as i8;
        let addr = self.regs.pc.wrapping_add(rel as u16);

        self.goto_in(bus, addr, false);
    }

    fn call_in(&mut self, bus: &mut Bus) {
        self.goto_in(bus, self.fetched_data, true);
    }

    fn rst_in(&mut self, bus: &mut Bus) {
        self.goto_in(bus, self.cur_inst.param as u16, true);
    }

    fn ret_in(&mut self, bus: &mut Bus) {
        if self.cur_inst.condition != CT::None {
            bus.cycle(1);
        }

        if self.check_cond() {
            let lo = self.stack_pop(bus);
            bus.cycle(1);
            let hi = self.stack_pop(bus);
            bus.cycle(1);

            let value = ((hi as u16) << 8) | (lo as u16);
            self.regs.pc = value;

            bus.cycle(1);
        }
    }

    #[inline(always)]
    fn reti_in(&mut self, bus: &mut Bus) {
        self.interrupt_master_enabled = true;
        self.ret_in(bus);
    }

    fn inc_in(&mut self, bus: &mut Bus) {
        let mut val = self.read_reg(self.cur_inst.r1).wrapping_add(1);

        if self.cur_inst.r1.is_16bit() {
            bus.cycle(1);
        }

        if self.cur_inst.r1 == RT::HL && self.cur_inst.mode == AM::Mem {
            let reg_hl = self.read_reg(RT::HL);
            val = ((bus.read(reg_hl)) as u16).wrapping_add(1);
            val &= 0xFF;
            bus.write(reg_hl, val as u8);
        } else {
            self.set_reg(self.cur_inst.r1, val);
            val = self.read_reg(self.cur_inst.r1);
        }

        if (self.cur_opcode & 0x03) == 0x03 {
            return;
        }

        self.regs.set_flag(Flag::Z, val == 0);
        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, (val & 0x0F) == 0);
    }

    fn dec_in(&mut self, bus: &mut Bus) {
        let mut val = self.read_reg(self.cur_inst.r1).wrapping_sub(1);

        if self.cur_inst.r1.is_16bit() {
            bus.cycle(1);
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
            return;
        }

        let flag_z = val == 0;
        let flag_h = (val & 0x0F) == 0x0F;

        self.regs.set_flag(Flag::Z, flag_z);
        self.regs.set_flag(Flag::N, true);
        self.regs.set_flag(Flag::H, flag_h);
    }

    fn add_in(&mut self, bus: &mut Bus) {
        let reg_1: u16 = self.read_reg(self.cur_inst.r1);
        let is_16bit: bool = self.cur_inst.r1.is_16bit();
        let is_sp: bool = self.cur_inst.r1 == RT::SP;
        let val: u32 = if is_sp {
            (reg_1 + self.fetched_data as i8 as u16) as u32
        } else {
            (reg_1 + self.fetched_data) as u32
        };

        if is_16bit {
            bus.cycle(1);
        }

        let (z, h, c) = if is_sp {
            (
                Some(false),
                (reg_1 & 0xF) + (self.fetched_data & 0xF) >= 0x10,
                (reg_1 & 0xFF) as i32 + (self.fetched_data & 0xFF) as i32 >= 0x100,
            )
        } else if is_16bit {
            let n: u32 = (reg_1 as u32) + (self.fetched_data as u32);
            (
                None,
                (reg_1 & 0xFFF) + (self.fetched_data & 0xFFF) >= 0x1000,
                n >= 0x10000,
            )
        } else {
            (
                Some((val & 0xFF) == 0),
                (reg_1 & 0xF) + (self.fetched_data & 0xF) >= 0x10,
                (reg_1 & 0xFF) as i32 + (self.fetched_data & 0xFF) as i32 >= 0x100,
            )
        };

        #[allow(clippy::identity_op)]
        self.set_reg(self.cur_inst.r1, (val & 0xFFFF) as u16);

        if let Some(flag_z) = z {
            self.regs.set_flag(Flag::Z, flag_z);
        }
        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, h);
        self.regs.set_flag(Flag::C, c);
    }

    fn adc_in(&mut self) {
        let u = self.fetched_data;
        let a = self.regs.a as u16;
        let c = self.regs.flag_c() as u16;

        self.regs.a = ((a + u + c) & 0xFF) as u8;

        let flag_z = self.regs.a == 0;
        let flag_h = ((a & 0xF) + (u & 0xF) + c) > 0xF;
        let flag_c = (a + u + c) > 0xFF;

        self.regs.set_flag(Flag::Z, flag_z);
        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, flag_h);
        self.regs.set_flag(Flag::C, flag_c);
    }

    fn sub_in(&mut self) {
        let reg_val = self.read_reg(self.cur_inst.r1);
        let val = reg_val.wrapping_sub(self.fetched_data);

        let flag_z = val == 0;
        let flag_h = (reg_val as i32 & 0xF).wrapping_sub(self.fetched_data as i32 & 0xF) < 0;
        let flag_c = (reg_val as i32).wrapping_sub(self.fetched_data as i32) < 0;

        self.set_reg(self.cur_inst.r1, val);

        self.regs.set_flag(Flag::Z, flag_z);
        self.regs.set_flag(Flag::N, true);
        self.regs.set_flag(Flag::H, flag_h);
        self.regs.set_flag(Flag::C, flag_c);
    }

    fn sbc_in(&mut self) {
        let flag_c = self.regs.flag_c();
        let val = (self.fetched_data + (flag_c as u16)) as u8;
        let reg_1 = self.read_reg(self.cur_inst.r1);

        let flag_z = (reg_1 - val as u16) == 0;
        let flag_h = (reg_1 as i32 & 0xF)
            .wrapping_sub(self.fetched_data as i32 & 0xF)
            .wrapping_sub(flag_c as i32)
            < 0;
        let flag_c = (reg_1 as i32)
            .wrapping_sub(self.fetched_data as i32)
            .wrapping_sub(flag_c as i32)
            < 0;

        self.set_reg(self.cur_inst.r1, reg_1 - val as u16);

        self.regs.set_flag(Flag::Z, flag_z);
        self.regs.set_flag(Flag::N, true);
        self.regs.set_flag(Flag::H, flag_h);
        self.regs.set_flag(Flag::C, flag_c);
    }

    fn and_in(&mut self) {
        self.regs.a &= self.fetched_data as u8;

        self.regs.set_flags(self.regs.a == 0, false, true, false);
    }

    fn xor_in(&mut self) {
        self.regs.a ^= (self.fetched_data & 0xFF) as u8;

        self.regs.set_flags(self.regs.a == 0, false, false, false);
    }

    #[allow(clippy::identity_op)]
    fn or_in(&mut self) {
        self.regs.a |= self.fetched_data as u8 & 0xFF;
        self.regs.set_flags(self.regs.a == 0, false, false, false);
    }

    fn cp_in(&mut self) {
        let a = self.regs.a as i32;
        let data = self.fetched_data as i32;

        let result = a.wrapping_sub(data);

        let flag_z = result == 0;
        let flag_n = true;
        let flag_h = ((a & 0x0F) - (data & 0x0F)) < 0;
        let flag_c = result < 0;

        self.regs.set_flags(flag_z, flag_n, flag_h, flag_c);
    }

    fn cb_in(&mut self, bus: &mut Bus) {
        let operation = self.fetched_data as u8;
        let reg = RT::decode(operation & 0b111);
        let mut reg_val = self.read_reg8(reg, bus);
        let bit = (operation >> 3) & 0b111;
        let bit_op = (operation >> 6) & 0b11;
        let flag_c = self.regs.flag_c();

        bus.cycle(1);

        if reg == RT::HL {
            bus.cycle(2);
        }

        match bit_op {
            1 => {
                // BIT
                let flag_z = reg_val & (1 << bit) == 0;
                self.regs.set_flag(Flag::Z, flag_z);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, true);
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                self.set_reg8(reg, reg_val, bus);
                return;
            }
            3 => {
                // SET
                reg_val |= !(1 << bit);
                self.set_reg8(reg, reg_val, bus);
                return;
            }
            _ => {}
        };

        match bit {
            0 => {
                // RLC
                let mut flag_c = false;
                #[allow(clippy::identity_op)]
                let mut result = (reg_val << 1) & 0xFF; // (reg_val << 1) & 0xFF;

                if (reg_val & (1 << 7)) != 0 {
                    result |= 1;
                    flag_c = true;
                }

                self.set_reg8(reg, result, bus);

                self.regs.set_flag(Flag::Z, result == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, flag_c);
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= old << 7;

                self.set_reg8(reg, reg_val, bus);

                self.regs.set_flag(Flag::Z, (!reg_val) != 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, (old & 1) != 0);
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val <<= 1;
                reg_val |= flag_c as u8;

                self.set_reg8(reg, reg_val, bus);

                self.regs.set_flag(Flag::Z, (!reg_val) != 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, (!!(old & 0x80)) != 0)
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val >>= 1;
                reg_val |= (flag_c as u8) << 7;

                self.set_reg8(reg, reg_val, bus);

                self.regs.set_flag(Flag::Z, reg_val == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, (old & 1) != 0);
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                self.set_reg8(reg, reg_val, bus);

                self.regs.set_flag(Flag::Z, reg_val == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, (!!(old & 0x80)) != 0);
            }
            5 => {
                // SRA
                let result = ((reg_val as i8) >> 1) as u8; // Perform arithmetic shift right, preserving the sign bit

                self.set_reg8(reg, result, bus);

                self.regs.set_flag(Flag::Z, result == 0);
                self.regs.set_flag(Flag::N, false);
                self.regs.set_flag(Flag::H, false);
                self.regs.set_flag(Flag::C, (reg_val & 1) != 0);
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);

                self.set_reg8(reg, reg_val, bus);
                self.regs.set_flag(Flag::Z, reg_val == 0); // Zero flag
                self.regs.set_flag(Flag::N, false); // Subtraction flag
                self.regs.set_flag(Flag::H, false); // Half-carry flag
                self.regs.set_flag(Flag::C, false); // Carry flag
            }
            7 => {
                // SRL
                let result = reg_val >> 1;

                self.set_reg8(reg, result, bus);

                self.regs.set_flag(Flag::Z, result == 0); // Zero flag
                self.regs.set_flag(Flag::N, false); // Subtraction flag
                self.regs.set_flag(Flag::H, false); // Half-carry flag
                self.regs.set_flag(Flag::C, (reg_val & 1) != 0); // Carry flag if LSB is 1
            }
            _ => panic!("INVALID CB INSTRUCTION: {:02X}", operation),
        };
    }

    fn pop_in(&mut self, bus: &mut Bus) {
        let lo = self.stack_pop(bus);
        bus.cycle(1);
        let hi = self.stack_pop(bus);
        bus.cycle(1);

        let result = ((hi as u16) << 8) | (lo as u16);
        let reg_1 = self.cur_inst.r1;

        self.set_reg(reg_1, result);

        if reg_1 == RT::AF {
            self.set_reg(reg_1, result & 0xFFF0)
        };
    }

    fn push_in(&mut self, bus: &mut Bus) {
        let hi = (self.read_reg(self.cur_inst.r1) >> 8) & 0xFF;
        bus.cycle(1);
        self.stack_push(hi as u8, bus);

        let lo = self.read_reg(self.cur_inst.r1) & 0xFF;
        bus.cycle(1);
        self.stack_push(lo as u8, bus);

        bus.cycle(1);
    }

    #[inline(always)]
    fn di_in(&mut self) {
        self.interrupt_master_enabled = false;
    }

    fn ei_in(&mut self) {
        self.enabling_ime = true;
    }

    fn rlca_in(&mut self) {
        let mut reg_a = self.regs.a;
        let flag_c = (reg_a >> 7) & 1 != 0;

        reg_a = (reg_a << 1) | flag_c as u8;
        self.regs.a = reg_a;

        self.regs.set_flags(false, false, false, flag_c);
    }

    fn rrca_in(&mut self) {
        let res = self.regs.a & 1;
        self.regs.a >>= 1;
        self.regs.a |= res << 7;

        self.regs.set_flags(false, false, false, res != 0);
    }

    fn rla_in(&mut self) {
        let tmp = self.regs.a;
        let flag_c = self.regs.flag_c() as u8;
        let c = (tmp >> 7) & 1;

        self.regs.a = (tmp << 1) | flag_c;
        self.regs.set_flags(false, false, false, c != 0);
    }

    fn rra_in(&mut self) {
        let carry = self.regs.flag_c() as u8;
        let new_carry = self.regs.a & 1;

        self.regs.a >>= 1;
        self.regs.a |= carry << 7;

        self.regs.set_flag(Flag::Z, false);
        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, false);
        self.regs.set_flag(Flag::C, new_carry != 0);
    }

    fn stop_in(&mut self) {
        panic!("STOP INSTRUCTION PROCESS");
    }

    fn daa_in(&mut self) {
        let mut adjust = 0;
        let mut carry = false;

        if self.regs.flag_h() || (!self.regs.flag_n() && (self.regs.a & 0xF) > 9) {
            adjust = 6;
        }

        if self.regs.flag_c() || (!self.regs.flag_n() && self.regs.a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        if self.regs.flag_n() {
            self.regs.a = self.regs.a.wrapping_sub(adjust);
        } else {
            self.regs.a = self.regs.a.wrapping_add(adjust);
        }

        self.regs.set_flag(Flag::Z, self.regs.a == 0);
        self.regs.set_flag(Flag::H, false);
        self.regs.set_flag(Flag::C, carry);
    }

    fn cpl_in(&mut self) {
        self.regs.a = !self.regs.a;

        self.regs.set_flag(Flag::N, true);
        self.regs.set_flag(Flag::H, true);
    }

    fn scf_in(&mut self) {
        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, false);
        self.regs.set_flag(Flag::C, true);
    }

    fn ccf_in(&mut self) {
        let flag_c = self.regs.flag_c();

        self.regs.set_flag(Flag::N, false);
        self.regs.set_flag(Flag::H, false);
        self.regs.set_flag(Flag::C, !flag_c);
    }

    fn halt_in(&mut self) {
        self.is_halted = true;
    }
}
