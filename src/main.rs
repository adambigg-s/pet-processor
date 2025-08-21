mod assembler;
mod cpu;
mod instructions;
mod memory;

use std::fmt::Debug;

use assembler::ProgramAssembler;
use cpu::Clock;
use cpu::Data;
use cpu::Processor;
use cpu::processor_run;
use instructions::Instruction;
use memory::MemoryBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBarStruct;

trait _FooBarTrait<T>
where
    T: Debug + Default + Clone + Copy + Sized,
{
}

const RAM_SIZE: usize = 128;
const CYCLE_LIMIT: usize = 100_000_000;
const REG_COUNT: usize = 8;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let mut processor = Processor::<REG_COUNT>::default();
    let mut clock = Clock::default();
    let mut ram = MemoryBlock::<RAM_SIZE, Data>::default();

    let mut assembler = ProgramAssembler::build(&mut ram);
    assembler.assemble_program(vec![
        vec![Instruction::LoadImm.into(), 3, 10],
        vec![Instruction::LoadImm.into(), 1, 0],
        vec![Instruction::LoadImm.into(), 2, 1],
        vec![Instruction::LoadImm.into(), 4, 0],
        vec![Instruction::Push.into(), 1],
        vec![Instruction::Push.into(), 2],
        vec![Instruction::DebugDumpReg.into()],
        vec![Instruction::Add.into(), 0, 1, 2],
        vec![Instruction::Push.into(), 0],
        vec![Instruction::Copy.into(), 1, 2],
        vec![Instruction::Copy.into(), 2, 0],
        vec![Instruction::Decrement.into(), 3],
        vec![Instruction::Compare.into(), 3, 4],
        vec![Instruction::JumpIf.into(), 16],
        vec![Instruction::Halt.into()],
    ]);

    processor_run(&mut processor, &mut ram, &mut clock);

    dbg!(&ram, &processor, &clock);
}
