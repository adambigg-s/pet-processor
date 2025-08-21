mod assembler;
mod bus;
mod cpu;
mod instructions;
mod memory;

use assembler::ProgramAssembler;
use bus::Bus;
use cpu::Clock;
use cpu::Data;
use cpu::Pointer;
use cpu::Processor;
use cpu::processor_run;
use instructions::Instruction;
use memory::MemoryBlock;

const RAM_SIZE: usize = 128;
const CYCLE_LIMIT: usize = 100_000_000;
const REG_COUNT: usize = 8;

fn main() {
    let mut processor = Processor::<REG_COUNT>::default();
    let mut clock = Clock::default();
    let mut ram = MemoryBlock::<RAM_SIZE, Data>::default();
    let mut bus = Bus::<Pointer, Data>::default();

    let mut assembler = ProgramAssembler::build(&mut ram);
    assembler.assemble_program(vec![
        vec![Instruction::LoadImm.into(), 0, 1],
        vec![Instruction::LoadImm.into(), 1, 3],
        vec![Instruction::Add.into(), 2, 0, 1],
        vec![Instruction::Halt.into()],
    ]);

    let cycle_start = std::time::Instant::now();
    processor_run(&mut processor, &mut ram, &mut bus, &mut clock);
    let elapsed = cycle_start.elapsed().as_secs_f32();
    println!("\x1b[2J\x1b[0H");
    dbg!(&ram);
    dbg!(&processor);
    dbg!(&elapsed);
    dbg!(&clock);
}
