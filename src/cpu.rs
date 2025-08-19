use crate::CYCLE_LIMIT;
use crate::Data;
use crate::Pointer;
use crate::memory::Addressable;
use crate::memory::MemoryBlock;

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Instruction {
    #[default]
    Halt,
    Null,
    EnumLength,
}

impl From<Data> for Instruction {
    fn from(value: Data) -> Self {
        if value < Instruction::EnumLength.into() {
            return unsafe { std::mem::transmute::<Data, Instruction>(value) };
        }
        panic!("this can only be explained by corrupted bytes");
    }
}

impl From<Instruction> for Data {
    fn from(value: Instruction) -> Self {
        value as Data
    }
}

#[derive(Debug, Default)]
pub struct Clock {
    tick: usize,
}

type RegisterArray<const R: usize, Data> = MemoryBlock<R, Data>;

#[derive(Debug)]
pub struct Processor<const R: usize, DataWidth, MemoryWidth> {
    program_counter: MemoryWidth,
    registers: RegisterArray<R, DataWidth>,
    halted: bool,
}

impl<const R: usize> Processor<R, Data, Pointer> {
    fn step<Memory: Addressable<Pointer, Data = Data>>(&mut self, ram: &mut Memory) {
        let instruction = self.fetch(ram).into();
        self.execute(ram, instruction);
    }

    fn fetch<Memory: Addressable<Pointer, Data = Data>>(&mut self, ram: &mut Memory) -> Data {
        let address = self.program_counter;
        self.program_counter += 1;
        ram.read(address)
    }

    fn execute<Memory: Addressable<Pointer, Data = Data>>(
        &mut self,
        ram: &mut Memory,
        instruction: Instruction,
    ) {
        match instruction {
            | Instruction::Halt => {
                self.halted = true;
            }
            | Instruction::Null => {}
            | _ => panic!("this can only be explained by corrupted bytes"),
        }
    }
}

impl<const R: usize, DataWidth, MemoryWidth> Default for Processor<R, DataWidth, MemoryWidth>
where
    DataWidth: Default + Copy,
    MemoryWidth: Default,
{
    fn default() -> Self {
        Self {
            program_counter: Default::default(),
            registers: Default::default(),
            halted: Default::default(),
        }
    }
}

pub fn processor_run<const M: usize, const R: usize>(
    cpu: &mut Processor<R, Data, Pointer>,
    ram: &mut MemoryBlock<M, Data>,
    clock: &mut Clock,
) {
    while !cpu.halted && clock.tick < CYCLE_LIMIT {
        cpu.step(ram);
        clock.tick += 1;
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
        let _ = Into::<Instruction>::into(Instruction::EnumLength as u8 + 1);
    }
}
