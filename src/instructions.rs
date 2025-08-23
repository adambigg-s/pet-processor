use crate::bus::Bus;
use crate::cpu::Data;
use crate::cpu::Pointer;
use crate::cpu::Processor;
use crate::memory::Addressable;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Instruction {
    #[default]
    Halt,
    Null,
    LoadImm,    // dstr, valu
    LoadMem,    // dstr, addr
    Copy,       // dstr, reg1
    Add,        // dstr, reg1, reg2
    Sub,        // dstr, reg1, reg2
    Mul,        // dstr, reg1, reg2
    Div,        // dstr, reg1, reg2
    Jump,       // addr
    JumpIfZero, // addr
    Push,       // reg1
    Pop,        // reg1
    Compare,    // reg1, reg2
    Increment,  // dstr
    Decrement,  // dstr
    EnumLength,
}

impl Instruction {
    pub fn operand_count(&self) -> usize {
        match self {
            | Instruction::Halt | Instruction::Null => 0,
            | Instruction::Jump
            | Instruction::JumpIfZero
            | Instruction::Push
            | Instruction::Pop
            | Instruction::Increment
            | Instruction::Decrement => 1,
            | Instruction::LoadImm | Instruction::LoadMem | Instruction::Copy | Instruction::Compare => 2,
            | Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div => 3,
            | _ => panic!("this can only be explained by corrupted bytes\ndecoded value: {self:?}"),
        }
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        if value >= Instruction::EnumLength.into() {
            panic!("this can only be explained by corrupted bytes\ndecoded value: {value}");
        }

        unsafe { std::mem::transmute::<u8, Instruction>(value) }
    }
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        value as u8
    }
}

pub fn halt<const R: usize>(cpu: &mut Processor<R>) {
    cpu.halted = true;
}

pub fn load_imm<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let val = cpu.operand_buffer.read_next();
    *cpu.registers.write(dst) = val;
}

pub fn load_mem<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    todo!()
}

pub fn copy<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let rg1 = cpu.operand_buffer.read_next();
    let val = cpu.registers.read(rg1);
    *cpu.registers.write(dst) = val;
}

pub fn add<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let rg1 = cpu.operand_buffer.read_next();
    let rg2 = cpu.operand_buffer.read_next();
    let va1 = cpu.registers.read(rg1);
    let va2 = cpu.registers.read(rg2);
    *cpu.registers.write(dst) = arithmetic::add(va1, va2);
}

pub fn sub<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let rg1 = cpu.operand_buffer.read_next();
    let rg2 = cpu.operand_buffer.read_next();
    let va1 = cpu.registers.read(rg1);
    let va2 = cpu.registers.read(rg2);
    *cpu.registers.write(dst) = arithmetic::sub(va1, va2);
}

pub fn mul<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let rg1 = cpu.operand_buffer.read_next();
    let rg2 = cpu.operand_buffer.read_next();
    let va1 = cpu.registers.read(rg1);
    let va2 = cpu.registers.read(rg2);
    *cpu.registers.write(dst) = arithmetic::mul(va1, va2);
}

pub fn div<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let rg1 = cpu.operand_buffer.read_next();
    let rg2 = cpu.operand_buffer.read_next();
    let va1 = cpu.registers.read(rg1);
    let va2 = cpu.registers.read(rg2);
    *cpu.registers.write(dst) = arithmetic::div(va1, va2);
}

pub fn jump<const R: usize>(cpu: &mut Processor<R>) {
    let adr = cpu.operand_buffer.read_next();
    cpu.program_counter = adr;
}

pub fn jump_if_zero<const R: usize>(cpu: &mut Processor<R>) {
    let adr = cpu.operand_buffer.read_next();
    if !cpu.flags.zero {
        cpu.program_counter = adr;
    }
}

pub fn push<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    let rg1 = cpu.operand_buffer.read_next();
    let val = cpu.registers.read(rg1);
    assert!(bus.is_avaliable());
    bus.dispatch_write(cpu.stack_pointer, val);
    cpu.stack_pointer -= 1;
}

pub fn pop<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    todo!()
}

pub fn compare<const R: usize>(cpu: &mut Processor<R>) {
    let rg1 = cpu.operand_buffer.read_next();
    let rg2 = cpu.operand_buffer.read_next();
    let va1 = cpu.registers.read(rg1);
    let va2 = cpu.registers.read(rg2);
    cpu.flags.reset();
    match logic::compare(va1, va2) {
        | logic::Ordering::Equal => cpu.flags.zero = true,
        | logic::Ordering::Less => cpu.flags.less = true,
        | logic::Ordering::Great => cpu.flags.great = true,
    }
}

pub fn increment<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let val = cpu.registers.read(dst);
    *cpu.registers.write(dst) = arithmetic::add(val, 1);
}

pub fn decrement<const R: usize>(cpu: &mut Processor<R>) {
    let dst = cpu.operand_buffer.read_next();
    let val = cpu.registers.read(dst);
    *cpu.registers.write(dst) = arithmetic::sub(val, 1);
}

pub mod logic {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub enum Ordering {
        Equal,
        Less,
        Great,
    }

    pub fn compare(x: u8, y: u8) -> Ordering {
        let diff = x ^ y;
        let mut out = Ordering::Equal;
        let mut mask = 1 << 7;
        while mask > 0 {
            if diff & mask != 0 {
                if x & mask != 0 {
                    out = Ordering::Great;
                }
                if y & mask != 0 {
                    out = Ordering::Less;
                }
            }
            mask >>= 1;
        }

        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn bit_cmp() {
            let x = 5;
            let y = 11;
            let z = 5;
            assert!(compare(x, y) == Ordering::Less);
            assert!(compare(y, x) == Ordering::Great);
            assert!(compare(x, z) == Ordering::Equal);
        }
    }
}

pub mod arithmetic {
    pub fn add(mut x: u8, mut y: u8) -> u8 {
        while y != 0 {
            let carry = x & y;
            x ^= y;
            y = carry << 1;
        }
        x
    }

    pub fn sub(x: u8, mut y: u8) -> u8 {
        y = add(!y, 1);
        add(x, y)
    }

    pub fn mul(mut x: u8, mut y: u8) -> u8 {
        let mut out = 0;
        while y != 0 {
            if y & 1 != 0 {
                out = add(out, x);
            }
            x <<= 1;
            y >>= 1;
        }
        out
    }

    pub fn div(mut x: u8, mut y: u8) -> u8 {
        assert!(y != 0);
        let mut q = 0;
        let mut mask = 1;
        while y <= x {
            y <<= 1;
            mask <<= 1;
        }
        while mask > 0 {
            if x >= y {
                x = sub(x, y);
                q |= mask;
            }
            y >>= 1;
            mask >>= 1;
        }
        q
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn ripple_adder() {
            let x = 13;
            let y = 33;
            assert!(add(x, y) == x + y);
        }

        #[test]
        fn ripple_subtractor() {
            let x = 33;
            let y = 13;
            assert!(sub(x, y) == x - y);
        }

        #[test]
        fn bit_mul() {
            let x = 13;
            let y = 9;
            assert!(mul(x, y) == x * y);
        }

        #[test]
        fn bit_div() {
            let x = 37;
            let y = 12;
            assert!(div(x, y) == x / y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_comparison)]
    fn boolean_algebra() {
        assert!(true == true & true);
        assert!(true != true & false);
        assert!(true == true | false);
        assert!(true == true | true);
        assert!(true == true ^ false);
        assert!(true != true ^ true);
        assert!(true != false ^ false);
        assert!(true != false);
    }

    #[test]
    fn enum_transmute() {
        assert!(Into::<Instruction>::into(0_u8) == Instruction::Halt);
        assert!(Into::<Instruction>::into(1_u8) == Instruction::Null);
    }

    #[test]
    fn u8_transmute() {
        assert!(Into::<u8>::into(Instruction::Halt) == 0);
        assert!(Into::<u8>::into(Instruction::Null) == 1);
    }

    #[test]
    #[should_panic]
    fn bad_transmute() {
        let _ = Into::<Instruction>::into(Instruction::EnumLength as u8);
    }
}
