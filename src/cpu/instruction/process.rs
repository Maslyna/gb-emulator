use crate::cpu::instruction::*;
use crate::cpu::*;
use crate::memory::*;

use AddressMode as AM;
use ConditionType as CT;
use InstructionType as IT;

pub fn process_instruction(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    match cpu.cur_inst.in_type {
        IT::None => panic!("INVALID INSTRUCTION: {:?}", cpu.cur_inst),
        IT::Nop => nop_in(),
        IT::Ld => ld_in(cpu, bus),
        IT::Inc => inc_in(cpu, bus),
        IT::Dec => dec_in(cpu, bus),
        IT::Rlca => todo!(),
        IT::Add => add_in(cpu),
        IT::Rrca => todo!(),
        IT::Stop => todo!(),
        IT::Rla => todo!(),
        IT::Rra => todo!(),
        IT::Daa => todo!(),
        IT::Cpl => todo!(),
        IT::Scf => todo!(),
        IT::Ccf => todo!(),
        IT::Halt => todo!(),
        IT::Adc => adc_in(cpu),
        IT::Sub => sub_in(cpu),
        IT::Sbc => sbc_in(cpu),
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
        IT::Ei => todo!(),
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
        let hflag =
            ((cpu.read_reg(cpu.cur_inst.r2) & 0xF) + (cpu.fetched_data & 0xF) >= 0x10) as i8;
        let cflag =
            ((cpu.read_reg(cpu.cur_inst.r2) & 0xFF) + (cpu.fetched_data & 0xFF) >= 0x100) as i8;

        cpu.regs.set_flags(0, 0, hflag, cflag);
        cpu.set_reg(
            cpu.cur_inst.r1,
            cpu.read_reg(cpu.cur_inst.r2) + cpu.fetched_data,
        );

        return emu_cycles;
    }

    cpu.set_reg(cpu.cur_inst.r1, cpu.fetched_data);
    return emu_cycles;
}

fn ldh_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;

    match cpu.cur_inst.r1 {
        RT::A => {
            cpu.set_reg(cpu.cur_inst.r1, bus.read(0xFF00 | cpu.fetched_data) as u16);
        }
        _ => {
            bus.write(0xFF00 | cpu.fetched_data, cpu.regs.a);
        }
    }
    emu_cycles += 1;
    return emu_cycles;
}

fn goto(cpu: &mut Cpu, address: u16, pushpc: bool, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    if cpu.check_cond() {
        if pushpc {
            cpu.stack_push16(cpu.regs.pc, bus);
            emu_cycles += 2;
        }

        cpu.regs.pc = address;
        emu_cycles += 1;
    }
    return emu_cycles;
}

#[inline(always)]
fn jp_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto(cpu, cpu.fetched_data, false, bus)
}

#[inline(always)]
fn call_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto(cpu, cpu.fetched_data, true, bus)
}

#[inline(always)]
fn rst_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    goto(cpu, cpu.cur_inst.param as u16, true, bus)
}

#[inline(always)]
fn jr_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let r: i8 = (cpu.fetched_data & 0xFF) as i8;
    let addr: u16 = (cpu.regs.pc as i16 + r as i16) as u16;

    goto(cpu, addr, false, bus)
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

        let value = bytes_to_word!(lo, hi);
        cpu.regs.pc = value;

        emu_cycles += 1;
    }
    return emu_cycles;
}

#[inline(always)]
fn reti_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    cpu._interrupt_master_enabled = true;
    ret_in(cpu, bus)
}

fn inc_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let mut val = cpu.read_reg(cpu.cur_inst.r1) + 1;

    if cpu.cur_inst.r1.is_16bit() {
        emu_cycles += 1;
    }

    if cpu.cur_inst.r1 == RT::HL && cpu.cur_inst.mode == AM::Mem {
        let reg_hl = cpu.read_reg(RT::HL);
        val = (bus.read(reg_hl) + 1) as u16;
        val &= 0xFF;
        bus.write(reg_hl, val as u8);
    } else {
        cpu.set_reg(cpu.cur_inst.r1, val);
        val = cpu.read_reg(cpu.cur_inst.r1);
    }

    if (cpu.cur_opcode & 0x03) == 0x03 {
        return emu_cycles;
    }

    cpu.regs
        .set_flags((val == 0) as i8, 0, ((val & 0xFF) == 0) as i8, -1);

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

    cpu.regs
        .set_flags((val == 0) as i8, 1, ((val & 0x0F) == 0x0F) as i8, -1);

    emu_cycles
}

fn add_in(cpu: &mut Cpu) -> i32 {
    let mut emu_cycles = 0;

    let reg_val: u16 = cpu.read_reg(cpu.cur_inst.r1);
    let is_16bit: bool = cpu.cur_inst.r1.is_16bit();
    let is_sp: bool = cpu.cur_inst.r1 == RT::SP;
    let val: u32 = if is_sp {
        (reg_val + cpu.fetched_data as i8 as u16) as u32
    } else {
        (reg_val + cpu.fetched_data) as u32
    };

    if is_16bit {
        emu_cycles += 1;
    }

    let (z, h, c) = if is_sp {
        (
            0,
            ((reg_val & 0xF) + (cpu.fetched_data & 0xF) >= 0x10) as i32,
            ((reg_val & 0xFF) as i32 + (cpu.fetched_data & 0xFF) as i32 > 0x100) as i32,
        )
    } else if is_16bit {
        let n: u32 = (reg_val as u32) + (cpu.fetched_data as u32);
        (
            -1,
            ((reg_val & 0xFFF) + (cpu.fetched_data & 0xFFF) >= 0x1000) as i32,
            (n >= 0x10000) as i32,
        )
    } else {
        (
            ((val & 0xFF) == 0) as i32,
            ((reg_val & 0xF) + (cpu.fetched_data & 0xF) >= 0x10) as i32,
            ((reg_val & 0xFF) as i32 + (cpu.fetched_data & 0xFF) as i32 >= 0x100) as i32,
        )
    };

    #[allow(clippy::identity_op)]
    cpu.set_reg(cpu.cur_inst.r1, val as u16 & 0xFFFF);
    cpu.regs.set_flags(z as i8, 0, h as i8, c as i8);

    emu_cycles
}

fn adc_in(cpu: &mut Cpu) -> i32 {
    let u = cpu.fetched_data;
    let a = cpu.regs.a as u16;
    let c = cpu.regs.flag_c() as u16;

    cpu.regs.a = ((a + u + c) & 0xFF) as u8;

    cpu.regs.set_flags(
        (cpu.regs.a == 0) as i8,
        0,
        (a & 0xF) as i8 + (u & 0xF) as i8 + (c > 0xF) as i8,
        (a + u + c > 0xFF) as i8,
    );

    0
}

fn sub_in(cpu: &mut Cpu) -> i32 {
    let reg_val = cpu.read_reg(cpu.cur_inst.r1);
    let val = reg_val.wrapping_sub(cpu.fetched_data);

    let z: i32 = (val == 0) as i32;
    let h: i32 = (((reg_val & 0xF) as i32 - (cpu.fetched_data & 0xF) as i32) < 0) as i32;
    let c: i32 = ((reg_val as i32) - (cpu.fetched_data as i32) < 0) as i32;

    cpu.set_reg(cpu.cur_inst.r1, val);
    cpu.regs.set_flags(z as i8, 1, h as i8, c as i8);

    0
}

fn sbc_in(cpu: &mut Cpu) -> i32 {
    let flag_c = cpu.regs.flag_c();
    let val = (cpu.fetched_data + (flag_c as u16)) as u8;
    let reg_val = cpu.read_reg(cpu.cur_inst.r1);

    let z: i32 = (reg_val - val as u16 == 0) as i32;
    let h: i32 =
        ((reg_val as i32 & 0xF) - (cpu.fetched_data as i32 & 0xF) - (flag_c as i32) < 0) as i32;
    let c: i32 = ((reg_val as i32) - (cpu.fetched_data as i32) - (flag_c as i32) < 0) as i32;

    cpu.set_reg(cpu.cur_inst.r1, reg_val - val as u16);
    cpu.regs.set_flags(z as i8, 1, h as i8, c as i8);

    0
}

fn and_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a &= cpu.fetched_data as u8;
    cpu.regs.set_flags((cpu.regs.a == 0) as i8, 0, 1, 0);

    0
}

fn or_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a |= cpu.fetched_data as u8;
    cpu.regs.set_flags((cpu.regs.a == 0) as i8, 0, 0, 0);

    0
}

fn cp_in(cpu: &mut Cpu) -> i32 {
    let z = ((cpu.regs.a as i32 - cpu.fetched_data as i32) == 0) as i8;
    let h = (((cpu.regs.a as i32 & 0x0F) - (cpu.fetched_data as i32 & 0x0F)) < 0) as i8;

    cpu.regs.set_flags(z, 1, h, (z < 0) as i8);

    0
}

fn cb_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let operation = cpu.fetched_data;
    let reg = RT::from(operation as u8 & 0b111);
    let mut reg_val = cpu.read_reg8(reg, bus);
    let bit = (operation as u8 >> 3) & 0b111;
    let bit_op = (operation as u8 >> 6) & 0b11;

    emu_cycles += 1;

    if reg == RT::HL {
        emu_cycles += 2;
    }

    match bit_op {
        1 => {
            // BIT
            let flag_z = !(reg_val & (1 << bit));
            cpu.regs.set_flags(flag_z as i8, 0, 1, -1);
            return emu_cycles;
        }
        2 => {
            // RST
            reg_val &= !(1 << bit);
            cpu.set_reg8(reg, reg_val, bus);
        }
        3 => {
            // SET
            reg_val |= !(1 << bit);
            cpu.set_reg8(reg, reg_val, bus);
            return emu_cycles;
        }
        _ => {}
    };

    let flag_c = cpu.regs.flag_c();

    match bit {
        0 => {
            // RLC
            let mut set_c = false;
            let mut result = reg_val << 1; // (reg_val << 1) & 0xFF;

            if (reg_val & (1 << 7)) != 0 {
                result |= 1;
                set_c = true;
            }

            cpu.set_reg8(reg, result, bus);
            cpu.regs.set_flags((result == 0) as i8, 0, 0, set_c as i8);
            return emu_cycles;
        }
        1 => {
            // RRC
            let old = reg_val;
            reg_val >>= 1;
            reg_val |= old << 7;

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
            return emu_cycles;
        }
        2 => {
            // RL
            let old = reg_val;
            reg_val <<= 1;
            reg_val |= flag_c as u8;

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs
                .set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
            return emu_cycles;
        }
        3 => {
            // RR
            let old = reg_val;
            reg_val >>= 1;
            reg_val |= (flag_c as u8) << 7;

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs.set_flags((!reg_val) as i8, 0, 0, (old & 1) as i8);
        }
        4 => {
            // SLA
            let old = reg_val;
            reg_val <<= 1;

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs
                .set_flags((!reg_val) as i8, 0, 0, !!(old & 0x80) as i8);
            return emu_cycles;
        }
        5 => {
            // SRA
            let u = (reg_val as i8 >> 1) as u8;

            cpu.set_reg8(reg, u, bus);
            cpu.regs.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
            return emu_cycles;
        }
        6 => {
            // SWAP
            reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);

            cpu.set_reg8(reg, reg_val, bus);
            cpu.regs.set_flags((reg_val == 0) as i8, 0, 0, 0);
            return emu_cycles;
        }
        7 => {
            // SRL
            let u = reg_val >> 1;

            cpu.set_reg8(reg, u, bus);
            cpu.regs.set_flags(!u as i8, 0, 0, (reg_val & 1) as i8);
            return emu_cycles;
        }
        _ => panic!("INVALID CB INSTRUCTION: {operation:02X}"),
    };

    emu_cycles
}

fn xor_in(cpu: &mut Cpu) -> i32 {
    cpu.regs.a ^= (cpu.fetched_data & 0xFF) as u8;
    cpu.regs.set_flags((cpu.regs.a == 0) as i8, 0, 0, 0);

    0
}

fn pop_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let lo = cpu.stack_pop(bus);
    emu_cycles += 1;
    let hi = cpu.stack_pop(bus);
    emu_cycles += 1;

    let val = bytes_to_word!(lo, hi);
    let reg_1 = cpu.cur_inst.r1;

    cpu.set_reg(reg_1, val);

    if reg_1 == RT::AF {
        cpu.set_reg(reg_1, val & 0xFFF0)
    };

    emu_cycles
}

fn push_in(cpu: &mut Cpu, bus: &mut Bus) -> i32 {
    let mut emu_cycles = 0;
    let hi = (cpu.read_reg(cpu.cur_inst.r1) >> 8) & 0xFF;
    emu_cycles += 1;
    cpu.stack_push(hi as u8, bus);

    let lo = cpu.read_reg(cpu.cur_inst.r2) & 0xFF;
    emu_cycles += 1;
    cpu.stack_push(lo as u8, bus);

    emu_cycles += 1;

    emu_cycles
}

#[inline(always)]
fn di_in(cpu: &mut Cpu) -> i32 {
    cpu._interrupt_master_enabled = false;

    0
}
