use crate::CYCLE_LIMIT;
use crate::RAM_SIZE;
use crate::instructions::Instruction;
use crate::instructions::arithmetic;
use crate::memory::Addressable;
use crate::memory::MemoryBlock;

#[derive(Debug, Default)]
pub struct Clock {
    tick: usize,
}

pub type Data = u8;
pub type Pointer = u8;

type RegisterArray<const R: usize, Data> = MemoryBlock<R, Data>;

#[derive(Debug, Default)]
pub struct ProcFlags {
    zero: bool,
    less: bool,
    great: bool,
}

impl ProcFlags {
    fn reset(&mut self) {
        self.zero = false;
        self.less = false;
        self.great = false;
    }
}

#[derive(Debug)]
pub struct Processor<const R: usize> {
    program_counter: Pointer,
    stack_pointer: Pointer,
    registers: RegisterArray<R, Data>,
    flags: ProcFlags,
    halted: bool,
}

pub fn processor_run<const M: usize, const R: usize>(
    cpu: &mut Processor<R>,
    ram: &mut MemoryBlock<M, Data>,
    clock: &mut Clock,
) {
    while !cpu.halted && clock.tick < CYCLE_LIMIT {
        cpu.step(ram);
        clock.tick += 1;
    }
}

impl<const R: usize> Processor<R> {
    fn step<Memory: Addressable<Pointer, Data = Data>>(&mut self, ram: &mut Memory) {
        let instruction = self.fetch(ram).into();
        self.execute(ram, instruction);
    }

    fn fetch<Memory: Addressable<Pointer, Data = Data>>(&mut self, ram: &mut Memory) -> Data {
        let output = ram.read(self.program_counter);
        self.program_counter += 1;
        output
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
            | Instruction::LoadImm => {
                let dst = self.fetch(ram);
                let val = self.fetch(ram);
                *self.registers.write(dst) = val;
            }
            | Instruction::LoadMem => {
                let dst = self.fetch(ram);
                let adr = self.fetch(ram);
                *self.registers.write(dst) = ram.read(Into::into(adr));
            }
            | Instruction::Copy => {
                let dst = self.fetch(ram);
                let rg1 = self.fetch(ram);
                *self.registers.write(dst) = self.registers.read(rg1);
            }
            | Instruction::Add => {
                let dst = self.fetch(ram);
                let rg1 = self.fetch(ram);
                let rg2 = self.fetch(ram);
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                *self.registers.write(dst) = arithmetic::add(va1, va2);
            }
            | Instruction::Sub => {
                let dst = self.fetch(ram);
                let rg1 = self.fetch(ram);
                let rg2 = self.fetch(ram);
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                *self.registers.write(dst) = arithmetic::sub(va1, va2);
            }
            | Instruction::Mul => {
                let dst = self.fetch(ram);
                let rg1 = self.fetch(ram);
                let rg2 = self.fetch(ram);
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                *self.registers.write(dst) = va1 * va2;
            }
            | Instruction::Div => {
                let dst = self.fetch(ram);
                let rg1 = self.fetch(ram);
                let rg2 = self.fetch(ram);
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                *self.registers.write(dst) = va1 / va2;
            }
            | Instruction::Jump => {
                let dst = self.fetch(ram);
                self.program_counter = Into::into(dst);
            }
            | Instruction::JumpIf => {
                let dst = self.fetch(ram);
                if !self.flags.zero {
                    self.program_counter = Into::into(dst);
                }
            }
            | Instruction::Push => {
                let rg1 = self.fetch(ram);
                *ram.write(self.stack_pointer) = self.registers.read(rg1);
                self.stack_pointer -= 1;
            }
            | Instruction::Pop => todo!(),
            | Instruction::Compare => {
                self.flags.reset();
                let rg1 = self.fetch(ram);
                let rg2 = self.fetch(ram);
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                match va1.cmp(&va2) {
                    | std::cmp::Ordering::Equal => self.flags.zero = true,
                    | std::cmp::Ordering::Greater => self.flags.great = true,
                    | std::cmp::Ordering::Less => self.flags.less = true,
                }
            }
            | Instruction::Increment => {
                let rg1 = self.fetch(ram);
                let val = self.registers.read(rg1);
                *self.registers.write(rg1) = arithmetic::add(val, 1);
            }
            | Instruction::Decrement => {
                let rg1 = self.fetch(ram);
                let val = self.registers.read(rg1);
                *self.registers.write(rg1) = arithmetic::sub(val, 1);
            }
            | Instruction::DebugRead => {
                let dst = self.fetch(ram);
                println!("value at register {dst}: {}", self.registers.read(dst));
            }
            | Instruction::DebugDumpReg => {
                println!("registers: {:?}", self.registers);
            }
            | Instruction::Null => {}
            | Instruction::EnumLength => panic!("this can only be explained by corrupted bytes"),
        }
    }
}

impl<const R: usize> Default for Processor<R> {
    fn default() -> Self {
        Self {
            program_counter: Default::default(),
            stack_pointer: RAM_SIZE as u8 - 1,
            registers: Default::default(),
            flags: Default::default(),
            halted: Default::default(),
        }
    }
}
