#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Instruction {
    #[default]
    Halt,
    Null,
    LoadImm,   // dstr, valu
    LoadMem,   // dstr, addr
    Copy,      // dstr, reg1
    Add,       // dstr, reg1, reg2
    Sub,       // dstr, reg1, reg2
    Mul,       // dstr, reg1, reg2
    Div,       // dstr, reg1, reg2
    Jump,      // addr
    JumpIf,    // addr
    Push,      // reg1
    Pop,       // reg1
    Compare,   // reg1, reg2
    Increment, // dstr
    Decrement, // dstr
    DebugRead,
    DebugDumpReg,
    EnumLength,
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
