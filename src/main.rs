mod cpu;
mod memory;

use std::fmt::Debug;

use cpu::Clock;
use cpu::Instruction;
use cpu::Processor;
use cpu::processor_run;
use memory::Addressable;
use memory::MemoryBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBarStruct;

trait _FooBarTrait<T>
where
    T: Debug + Default + Clone + Copy + Sized,
{
}

const RAM_SIZE: usize = 32;
const CYCLE_LIMIT: usize = 10_000_000;
const REG_COUNT: usize = 8;

type Data = u8;
type Pointer = u8;

fn main() {
    let mut processor = Processor::<REG_COUNT, Data, Pointer>::default();
    let mut clock = Clock::default();
    let mut ram = MemoryBlock::<RAM_SIZE, Data>::default();

    *ram.write(0_u8) = Instruction::Null.into();

    processor_run(&mut processor, &mut ram, &mut clock);

    dbg!(&ram);
    dbg!(&processor);
    dbg!(&clock);
}
