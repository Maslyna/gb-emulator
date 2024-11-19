use self::instruction::{AddressMode as AM, ConditionType as CT, InstructionType as IT};
use self::regs::CpuFlag as Flag;
use crate::cpu::*;
use crate::memory::*;

pub fn process(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    match cpu.cur_inst.in_type {
        IT::None => panic!("INVALID INSTRUCTION: {:?}", cpu.cur_inst),
        IT::Nop => nop_in(),
        IT::Ld => ld_in(cpu, bus),
        IT::Inc => inc_in(cpu, bus),
        IT::Dec => dec_in(cpu, bus),
        IT::Add => add_in(cpu),
        IT::Sub => sub_in(cpu),
        IT::Sbc => sbc_in(cpu),
        IT::Rlca => rlca_in(cpu),
        IT::Rrca => rrca_in(cpu),
        IT::Stop => stop_in(cpu),
        IT::Rla => rla_in(cpu),
        IT::Rra => rra_in(cpu),
        IT::Daa => daa_in(cpu),
        IT::Cpl => cpl_in(cpu),
        IT::Scf => scf_in(cpu),
        IT::Ccf => ccf_in(cpu),
        IT::Halt => halt_in(cpu),
        IT::Adc => adc_in(cpu),
        IT::And => and_in(cpu),
        IT::Xor => xor_in(cpu),
        IT::Or => or_in(cpu),
        IT::Cp => cp_in(cpu),
        IT::Jr => jr_in(cpu, bus),
        IT::Pop => pop_in(cpu, bus),
        IT::Jp => jp_in(cpu, bus),
        IT::Push => push_in(cpu, bus),
        IT::Ret => ret_in(cpu, bus),
        IT::Reti => reti_in(cpu, bus),
        IT::Cb => cb_in(cpu, bus),
        IT::Call => call_in(cpu, bus),
        IT::Ldh => ldh_in(cpu, bus),
        IT::Jphl => todo!(),
        IT::Di => di_in(cpu),
        IT::Ei => ei_in(cpu),
        IT::Rst => rst_in(cpu, bus),
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
fn nop_in() -> i32 {
    0
}

fn ld_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    if cpu.dest_is_mem {
        //e.g.: LD (BC) A
        if cpu.cur_inst.r2.is_16bit() {
            emu_cycles += 1;
            bus.write16(cpu.mem_dest, cpu.fetched_data);
        } else {
            bus.write(cpu.mem_dest, cpu.fetched_data as u8);
        }

        emu_cycles += 1;

        return emu_cycles;
    }

    if cpu.cur_inst.mode == AM::HLRegsSP {
        let reg_val = cpu.read_reg(cpu.cur_inst.r2);
        let offset = cpu.fetched_data;
        let hflag = ((reg_val & 0xF) + (offset & 0xF)) >= 0x10;
        let cflag = (reg_val + offset) >= 0x100;

        cpu.regs._set_flags(false, false, hflag, cflag);
        cpu.set_reg(cpu.cur_inst.r1, reg_val.wrapping_add(offset as u8 as u16));

        return emu_cycles;
    }

    cpu.set_reg(cpu.cur_inst.r1, cpu.fetched_data);
    emu_cycles
}

fn ldh_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    match cpu.cur_inst.r1 {
        RT::A => {
            cpu.set_reg(cpu.cur_inst.r1, bus.read(0xFF00 | cpu.fetched_data) as u16);
        }
        _ => {
            bus.write(cpu.mem_dest, cpu.regs.a);
        }
    }

    1
}

fn goto_in(cpu: &mut Cpu, address: u16, pushpc: bool, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    if cpu.check_cond() {
        if pushpc {
            cpu.stack_push16(cpu.regs.pc, bus);
            emu_cycles += 2;
        }

        cpu.regs.pc = address;
        emu_cycles += 1;
    }
    emu_cycles
}

fn jp_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto_in(cpu, cpu.fetched_data, false, bus)
}

fn jr_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let rel = (cpu.fetched_data & 0xFF) as i8;
    let addr = cpu.regs.pc.wrapping_add(rel as u16);

    goto_in(cpu, addr, false, bus)
}

fn call_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto_in(cpu, cpu.fetched_data, true, bus)
}

fn rst_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto_in(cpu, cpu.cur_inst.param as u16, true, bus)
}

fn ret_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;

    if cpu.cur_inst.condition != CT::None {
        emu_cycles += 1;
    }

    if cpu.check_cond() {
        let lo = cpu.stack_pop(bus);
        emu_cycles += 1;
        let hi = cpu.stack_pop(bus);
        emu_cycles += 1;

        let value = ((hi as u16) << 8) | (lo as u16);
        cpu.regs.pc = value;

        emu_cycles += 1;
    }
    emu_cycles
}

#[inline(always)]
fn reti_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    cpu.interrupt_master_enabled = true;
    ret_in(cpu, bus)
}

fn inc_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let mut val = cpu.read_reg(cpu.cur_inst.r1).wrapping_add(1);

    if cpu.cur_inst.r1.is_16bit() {
        emu_cycles += 1;
    }

    if cpu.cur_inst.r1 == RT::HL && cpu.cur_inst.mode == AM::Mem {
        let reg_hl = cpu.read_reg(RT::HL);
        val = ((bus.read(reg_hl)) as u16).wrapping_add(1);
        val &= 0xFF;
        bus.write(reg_hl, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.r1, val);
        val = cpu.read_reg(cpu.cur_inst.r1);
    }

    if (cpu.cur_opcode & 0x03) == 0x03 {
        return emu_cycles;
    }

    cpu.regs.set_flag(Flag::Z, val == 0);
    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, (val & 0x0F) == 0);

    emu_cycles
}

fn dec_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let mut val = cpu.read_reg(cpu.cur_inst.r1) - 1;

    if cpu.cur_inst.r1.is_16bit() {
        emu_cycles += 1;
    }

    if cpu.cur_inst.r1 == RT::HL && cpu.cur_inst.mode == AM::Mem {
        let reg_hl = cpu.read_reg(RT::HL);
        val = (bus.read(reg_hl) - 1) as u16;
        bus.write(reg_hl, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.r1, val);
        val = cpu.read_reg(cpu.cur_inst.r1);
    }

    if (cpu.cur_opcode & 0x03) == 0x03 {
        return emu_cycles;
    }

    let flag_z = val == 0;
    let flag_h = (val & 0x0F) == 0x0F;

    cpu.regs.set_flag(Flag::Z, flag_z);
    cpu.regs.set_flag(Flag::N, true);
    cpu.regs.set_flag(Flag::H, flag_h);

    emu_cycles
}

fn add_in(cpu: &mut Cpu) -> i32 {
    let mut emu_cycles = 0;

    let reg_1: u16 = cpu.read_reg(cpu.cur_inst.r1);
    let is_16bit: bool = cpu.cur_inst.r1.is_16bit();
    let is_sp: bool = cpu.cur_inst.r1 == RT::SP;
    let val: u32 = if is_sp {
        (reg_1 + cpu.fetched_data as i8 as u16) as u32
    } else {
        (reg_1 + cpu.fetched_data) as u32
    };

    if is_16bit {
        emu_cycles += 1;
    }

    let (z, h, c) = if is_sp {
        (
            Some(false),
            (reg_1 & 0xF) + (cpu.fetched_data & 0xF) >= 0x10,
            (reg_1 & 0xFF) as i32 + (cpu.fetched_data & 0xFF) as i32 >= 0x100,
        )
    } else if is_16bit {
        let n: u32 = (reg_1 as u32) + (cpu.fetched_data as u32);
        (
            None,
            (reg_1 & 0xFFF) + (cpu.fetched_data & 0xFFF) >= 0x1000,
            n >= 0x10000,
        )
    } else {
        (
            Some((val & 0xFF) == 0),
            (reg_1 & 0xF) + (cpu.fetched_data & 0xF) >= 0x10,
            (reg_1 & 0xFF) as i32 + (cpu.fetched_data & 0xFF) as i32 >= 0x100,
        )
    };

    #[allow(clippy::identity_op)]
    cpu.set_reg(cpu.cur_inst.r1, (val & 0xFFFF) as u16);

    if let Some(flag_z) = z {
        cpu.regs.set_flag(Flag::Z, flag_z);
    }
    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, h);
    cpu.regs.set_flag(Flag::C, c);

    emu_cycles
}

fn adc_in(cpu: &mut Cpu) -> i32 {
    let u = cpu.fetched_data;
    let a = cpu.regs.a as u16;
    let c = cpu.regs.flag_c() as u16;

    cpu.regs.a = ((a + u + c) & 0xFF) as u8;

    let flag_z = cpu.regs.a == 0;
    let flag_h = ((a & 0xF) + (u & 0xF) + c) > 0;
    let flag_c = (a + u + c) > 0xFF;

    cpu.regs.set_flag(Flag::Z, flag_z);
    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, flag_h);
    cpu.regs.set_flag(Flag::C, flag_c);

    0
}

fn sub_in(cpu: &mut Cpu) -> i32 {
    let reg_val = cpu.read_reg(cpu.cur_inst.r1);
    let val = reg_val.wrapping_sub(cpu.fetched_data);

    let flag_z = val == 0;
    let flag_h = (reg_val as i32 & 0xF).wrapping_sub(cpu.fetched_data as i32 & 0xF) < 0;
    let flag_c = (reg_val as i32).wrapping_sub(cpu.fetched_data as i32) < 0;

    cpu.set_reg(cpu.cur_inst.r1, val);

    cpu.regs.set_flag(Flag::Z, flag_z);
    cpu.regs.set_flag(Flag::N, true);
    cpu.regs.set_flag(Flag::H, flag_h);
    cpu.regs.set_flag(Flag::C, flag_c);

    0
}

fn sbc_in(cpu: &mut Cpu) -> i32 {
    let flag_c = cpu.regs.flag_c();
    let val = (cpu.fetched_data + (flag_c as u16)) as u8;
    let reg_1 = cpu.read_reg(cpu.cur_inst.r1);

    let flag_z = (reg_1 - val as u16) == 0;
    let flag_h = (reg_1 as i32 & 0xF)
        .wrapping_sub(cpu.fetched_data as i32 & 0xF)
        .wrapping_sub(flag_c as i32)
        < 0;
    let flag_c = (reg_1 as i32)
        .wrapping_sub(cpu.fetched_data as i32)
        .wrapping_sub(flag_c as i32)
        < 0;

    cpu.set_reg(cpu.cur_inst.r1, reg_1 - val as u16);

    cpu.regs.set_flag(Flag::Z, flag_z);
    cpu.regs.set_flag(Flag::N, true);
    cpu.regs.set_flag(Flag::H, flag_h);
    cpu.regs.set_flag(Flag::C, flag_c);

    0
}

fn and_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a &= cpu.fetched_data as u8;

    cpu.regs._set_flags(cpu.regs.a == 0, false, true, false);

    0
}

fn xor_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a ^= (cpu.fetched_data & 0xFF) as u8;

    cpu.regs._set_flags(cpu.regs.a == 0, false, false, false);

    0
}

#[allow(clippy::identity_op)]
fn or_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a |= cpu.fetched_data as u8 & 0xFF;
    cpu.regs._set_flags(cpu.regs.a == 0, false, false, false);

    0
}

fn cp_in(cpu: &mut Cpu) -> i32 {
    let a = cpu.regs.a as i32;
    let data = cpu.fetched_data as i32;

    let result = a.wrapping_sub(data);

    let flag_z = result == 0;
    let flag_n = true;
    let flag_h = ((a & 0x0F) - (data & 0x0F)) < 0;
    let flag_c = result < 0;

    cpu.regs._set_flags(flag_z, flag_n, flag_h, flag_c);

    0
}

fn cb_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let operation = cpu.fetched_data;
    let reg = RT::from(operation as u8 & 0b111);
    let mut reg_val = cpu.read_reg8(reg, bus);
    let bit = ((operation >> 3) & 0b111) as u8;
    let bit_op = ((operation >> 6) & 0b11) as u8;
    let flag_c = cpu.regs.flag_c();

    emu_cycles += if reg == RT::HL { 3 } else { 1 };

    match bit_op {
        1 => {
            // BIT
            let flag_z = !(reg_val & (1 << bit));
            cpu.regs.set_flag(Flag::Z, flag_z != 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, true);
            return emu_cycles;
        }
        2 => {
            // RST
            reg_val &= !(1 << bit);
            cpu.set_reg8(reg, reg_val, bus);
            return emu_cycles;
        }
        3 => {
            // SET
            reg_val |= !(1 << bit);
            cpu.set_reg8(reg, reg_val, bus);
            return emu_cycles;
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

            cpu.set_reg8(reg, result, bus);

            cpu.regs.set_flag(Flag::Z, result == 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, flag_c);

            return emu_cycles;
        }
        1 => {
            // RRC
            let old = reg_val;
            reg_val >>= 1;
            reg_val |= old << 7;

            cpu.set_reg8(reg, reg_val, bus);

            cpu.regs.set_flag(Flag::Z, (!reg_val) != 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, (old & 1) != 0);

            return emu_cycles;
        }
        2 => {
            // RL
            let old = reg_val;
            reg_val <<= 1;
            reg_val |= flag_c as u8;

            cpu.set_reg8(reg, reg_val, bus);

            cpu.regs.set_flag(Flag::Z, (!reg_val) != 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, (!!(old & 0x80)) != 0);
            return emu_cycles;
        }
        3 => {
            // RR
            let old = reg_val;
            reg_val >>= 1;
            reg_val |= (flag_c as u8) << 7;

            cpu.set_reg8(reg, reg_val, bus);

            cpu.regs.set_flag(Flag::Z, reg_val == 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, (old & 1) != 0);
        }
        4 => {
            // SLA
            let old = reg_val;
            reg_val <<= 1;

            cpu.set_reg8(reg, reg_val, bus);

            cpu.regs.set_flag(Flag::Z, reg_val == 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, (!!(old & 0x80)) != 0);

            return emu_cycles;
        }
        5 => {
            // SRA
            let result = ((reg_val as i8) >> 1) as u8; // Perform arithmetic shift right, preserving the sign bit

            cpu.set_reg8(reg, result, bus);

            cpu.regs.set_flag(Flag::Z, result == 0);
            cpu.regs.set_flag(Flag::N, false);
            cpu.regs.set_flag(Flag::H, false);
            cpu.regs.set_flag(Flag::C, (reg_val & 1) != 0);

            return emu_cycles;
        }
        6 => {
            // SWAP
            reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs.set_flag(Flag::Z, reg_val == 0); // Zero flag
            cpu.regs.set_flag(Flag::N, false); // Subtraction flag
            cpu.regs.set_flag(Flag::H, false); // Half-carry flag
            cpu.regs.set_flag(Flag::C, false); // Carry flag

            return emu_cycles;
        }
        7 => {
            // SRL
            let result = reg_val >> 1;

            cpu.set_reg8(reg, result, bus);

            cpu.regs.set_flag(Flag::Z, result == 0); // Zero flag
            cpu.regs.set_flag(Flag::N, false); // Subtraction flag
            cpu.regs.set_flag(Flag::H, false); // Half-carry flag
            cpu.regs.set_flag(Flag::C, (reg_val & 1) != 0); // Carry flag if LSB is 1

            return emu_cycles;
        }
        _ => panic!("INVALID CB INSTRUCTION: {:02X}", operation),
    };

    emu_cycles
}

fn pop_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let lo = cpu.stack_pop(bus);
    emu_cycles += 1;
    let hi = cpu.stack_pop(bus);
    emu_cycles += 1;

    let result = ((hi as u16) << 8) | (lo as u16);
    let reg_1 = cpu.cur_inst.r1;

    cpu.set_reg(reg_1, result);

    if reg_1 == RT::AF {
        cpu.set_reg(reg_1, result & 0xFFF0)
    };

    emu_cycles
}

fn push_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let hi = (cpu.read_reg(cpu.cur_inst.r1) >> 8) & 0xFF;
    emu_cycles += 1;
    cpu.stack_push(hi as u8, bus);

    let lo = cpu.read_reg(cpu.cur_inst.r1) & 0xFF;
    emu_cycles += 1;
    cpu.stack_push(lo as u8, bus);

    emu_cycles += 1;

    emu_cycles
}

#[inline(always)]
fn di_in(cpu: &mut Cpu) -> i32 {
    cpu.interrupt_master_enabled = false;

    0
}

fn ei_in(cpu: &mut Cpu) -> i32 {
    cpu.enabling_ime = true;

    0
}

fn rlca_in(cpu: &mut Cpu) -> i32 {
    let mut reg_a = cpu.regs.a;
    let flag_c = (reg_a >> 7) & 1 != 0;

    reg_a = (reg_a << 1) | flag_c as u8;
    cpu.regs.a = reg_a;

    cpu.regs._set_flags(false, false, false, flag_c);

    0
}

fn rrca_in(cpu: &mut Cpu) -> i32 {
    let res = cpu.regs.a & 1;
    cpu.regs.a >>= 1;
    cpu.regs.a |= res << 7;

    cpu.regs._set_flags(false, false, false, res != 0);

    0
}

fn rla_in(cpu: &mut Cpu) -> i32 {
    let tmp = cpu.regs.a;
    let flag_c = cpu.regs.flag_c() as u8;
    let c = (tmp >> 7) | 1;

    cpu.regs.a = (tmp << 1) | flag_c;
    cpu.regs._set_flags(false, false, false, c != 0);

    0
}

fn rra_in(cpu: &mut Cpu) -> i32 {
    let carry = cpu.regs.flag_c() as u8;
    let new_carry = cpu.regs.a & 1;

    cpu.regs.a >>= 1;
    cpu.regs.a |= carry << 7;

    cpu.regs.set_flag(Flag::Z, false);
    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, false);
    cpu.regs.set_flag(Flag::C, new_carry != 0);

    0
}

fn stop_in(_cpu: &mut Cpu) -> i32 {
    panic!("STOP INSTRUCTION PROCESS");
}

fn daa_in(cpu: &mut Cpu) -> i32 {
    let mut adjust = 0;
    let mut carry = false;

    if cpu.regs.flag_h() || (!cpu.regs.flag_n() && (cpu.regs.a & 0xF) > 9) {
        adjust = 6;
    }

    if cpu.regs.flag_c() || (!cpu.regs.flag_n() && cpu.regs.a > 0x99) {
        adjust |= 0x60;
        carry = true;
    }

    if cpu.regs.flag_n() {
        cpu.regs.a = cpu.regs.a.wrapping_sub(adjust);
    } else {
        cpu.regs.a = cpu.regs.a.wrapping_add(adjust);
    }

    cpu.regs.set_flag(Flag::Z, cpu.regs.a == 0);
    cpu.regs.set_flag(Flag::H, false);
    cpu.regs.set_flag(Flag::C, carry);

    0
}

fn cpl_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a = !cpu.regs.a;

    cpu.regs.set_flag(Flag::N, true);
    cpu.regs.set_flag(Flag::H, true);

    0
}

fn scf_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, false);
    cpu.regs.set_flag(Flag::C, true);

    0
}

fn ccf_in(cpu: &mut Cpu) -> i32 {
    let flag_c = cpu.regs.flag_c();

    cpu.regs.set_flag(Flag::N, false);
    cpu.regs.set_flag(Flag::H, false);
    cpu.regs.set_flag(Flag::C, !flag_c);

    0
}

fn halt_in(cpu: &mut Cpu) -> i32 {
    cpu.is_halted = true;

    0
}
