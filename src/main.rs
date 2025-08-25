mod assembler;
mod bus;
mod clock;
mod cpu;
mod instructions;
mod memory;

use assembler::ProgramAssembler;
use bus::Bus;
use clock::Clock;
use cpu::_processor_run_debug;
use cpu::Data;
use cpu::Pointer;
use cpu::Processor;
use cpu::processor_run;
use instructions::Instruction;
use memory::MemoryBlock;

const RAM_SIZE: usize = 512;
const CYCLE_LIMIT: usize = 100_000_000;
const REG_COUNT: usize = 8;

fn main() {
    let mut processor = Processor::<REG_COUNT>::default();
    let mut clock = Clock::default();
    let mut ram = MemoryBlock::<RAM_SIZE, Data>::default();
    let mut bus = Bus::<Pointer, Data>::default();

    let mut assembler = ProgramAssembler::build(&mut ram);
    // first 10 fibonacci numbers
    assembler.assemble_program(vec![
        vec![Instruction::LoadImm.into(), 3, 10],
        vec![Instruction::LoadImm.into(), 1, 0],
        vec![Instruction::LoadImm.into(), 2, 1],
        vec![Instruction::LoadImm.into(), 4, 0],
        vec![Instruction::Push.into(), 1],
        vec![Instruction::Push.into(), 2],
        vec![Instruction::Add.into(), 0, 1, 2],
        vec![Instruction::Push.into(), 0],
        vec![Instruction::Copy.into(), 1, 2],
        vec![Instruction::Copy.into(), 2, 0],
        vec![Instruction::Decrement.into(), 3],
        vec![Instruction::Compare.into(), 3, 4],
        vec![Instruction::JumpIfZero.into(), 16],
        vec![Instruction::Halt.into()],
    ]);
    // 5 factorial
    assembler.assemble_program(vec![
        vec![Instruction::LoadImm.into(), 0, 5],
        vec![Instruction::LoadImm.into(), 1, 1],
        vec![Instruction::LoadImm.into(), 2, 0],
        vec![Instruction::Mul.into(), 1, 1, 0],
        vec![Instruction::Decrement.into(), 0],
        vec![Instruction::Compare.into(), 0, 2],
        vec![Instruction::JumpIfZero.into(), 9],
        vec![Instruction::Push.into(), 1],
        vec![Instruction::Halt.into()],
    ]);
    // function jump and return
    assembler.assemble_program(vec![
        /* main */
        vec![Instruction::LoadImm.into(), 0, 33],
        vec![Instruction::LoadImm.into(), 1, 10],
        vec![Instruction::Push.into(), 1],
        vec![Instruction::Jump.into(), 11],
        vec![Instruction::Halt.into()],
        /* function to double a number */
        vec![Instruction::LoadImm.into(), 1, 2],
        vec![Instruction::Mul.into(), 0, 0, 1],
        vec![Instruction::Ret.into()],
    ]);

    let cycle_start = std::time::Instant::now();
    _processor_run_debug(&mut processor, &mut ram, &mut bus, &mut clock);
    processor_run(&mut processor, &mut ram, &mut bus, &mut clock);
    let elapsed = cycle_start.elapsed().as_secs_f32();
    println!("\x1b[2J\x1b[0H");
    dbg!(&ram);
    dbg!(&processor);
    dbg!(&elapsed);
    dbg!(&clock);
}
