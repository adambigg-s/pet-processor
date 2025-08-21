use crate::CYCLE_LIMIT;
use crate::bus::Bus;
use crate::instructions::Instruction;
use crate::instructions::arithmetic;
use crate::memory::Addressable;
use crate::memory::MemoryBlock;
use crate::memory::memory_cycle;

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
struct OperandBuffer<const N: usize, Data> {
    operands: MemoryBlock<N, Option<Data>>,
    required: usize,
    fetched: usize,
    reader_head: usize,
}

impl<const N: usize, Data> Default for OperandBuffer<N, Data>
where
    Data: Default + Copy,
{
    fn default() -> Self {
        Self {
            operands: MemoryBlock::default(),
            required: Default::default(),
            fetched: Default::default(),
            reader_head: Default::default(),
        }
    }
}

impl<const N: usize, Data> OperandBuffer<N, Data>
where
    Data: Copy,
{
    fn push(&mut self, operand: Data) {
        *self.operands.write(self.fetched) = Some(operand);
        self.fetched += 1;
    }

    fn is_full(&mut self) -> bool {
        self.fetched == self.required
    }

    fn read_next(&mut self) -> Data {
        let out = self.operands.read(self.reader_head);
        self.reader_head += 1;
        out.expect("uninitialized operand - buffer too short")
    }

    fn reset(&mut self) {
        self.fetched = Default::default();
        self.required = Default::default();
        self.reader_head = Default::default();
    }
}

#[derive(Debug, Default)]
enum ProcState {
    #[default]
    Idle,
    FetchInstruction,
    Decode,
    Execute,
    WriteBack,
}

#[derive(Debug, Default)]
pub struct Processor<const R: usize> {
    program_counter: Pointer,
    stack_pointer: Pointer,
    registers: RegisterArray<R, Data>,
    flags: ProcFlags,
    halted: bool,
    state: ProcState,
    current_instruction: Instruction,
    operand_buffer: OperandBuffer<R, Data>,
}

pub fn processor_run<const M: usize, const R: usize>(
    cpu: &mut Processor<R>,
    ram: &mut MemoryBlock<M, Data>,
    bus: &mut Bus<Pointer, Data>,
    clock: &mut Clock,
) {
    println!("\x1b[2J");

    while !cpu.halted && clock.tick < CYCLE_LIMIT {
        println!("\x1b[0H");
        dbg!(&ram);
        dbg!(&cpu);
        dbg!(&bus);
        std::thread::sleep(std::time::Duration::from_millis(50));

        memory_cycle(ram, bus);

        println!("\x1b[0H");
        dbg!(&ram);
        dbg!(&cpu);
        dbg!(&bus);
        std::thread::sleep(std::time::Duration::from_millis(50));

        processor_cycle(cpu, bus);

        println!("\x1b[0H");
        dbg!(&ram);
        dbg!(&cpu);
        dbg!(&bus);
        std::thread::sleep(std::time::Duration::from_millis(50));

        clock.tick += 1;
    }
}

fn processor_cycle<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    match cpu.state {
        | ProcState::Idle => {
            cpu.initiate_fetch(bus);
            cpu.state = ProcState::FetchInstruction;
        }
        | ProcState::FetchInstruction => {
            let Some(instruction) = bus.read_data()
            else {
                return;
            };

            cpu.current_instruction = instruction.into();
            cpu.operand_buffer.reset();
            cpu.operand_buffer.required = Instruction::operand_count(&instruction.into());
            cpu.state = ProcState::Decode;
        }
        | ProcState::Decode => {
            if let Some(operand) = bus.read_data() {
                cpu.operand_buffer.push(operand);
            }

            if !cpu.operand_buffer.is_full() {
                cpu.initiate_fetch(bus);
            }
            else {
                cpu.state = ProcState::Execute;
            }
        }
        | ProcState::Execute => {
            cpu.execute();
            cpu.state = ProcState::WriteBack;
        }
        | ProcState::WriteBack => {
            cpu.current_instruction = Instruction::Null;
            cpu.state = ProcState::Idle;
        }
    }
}

impl<const R: usize> Processor<R> {
    fn initiate_fetch(&mut self, bus: &mut Bus<Pointer, Data>) {
        bus.dispatch_read(self.program_counter);
        self.program_counter += 1;
    }

    fn execute(&mut self) {
        match self.current_instruction {
            | Instruction::Halt => {
                self.halted = true;
            }
            | Instruction::LoadImm => {
                let dst = self.operand_buffer.read_next();
                let val = self.operand_buffer.read_next();
                *self.registers.write(dst) = val;
            }
            | Instruction::Add => {
                let dst = self.operand_buffer.read_next();
                let rg1 = self.operand_buffer.read_next();
                let rg2 = self.operand_buffer.read_next();
                let va1 = self.registers.read(rg1);
                let va2 = self.registers.read(rg2);
                *self.registers.write(dst) = arithmetic::add(va1, va2);
            }
            | Instruction::Null => todo!(),
            | Instruction::LoadMem => todo!(),
            | Instruction::Copy => todo!(),
            | Instruction::Sub => todo!(),
            | Instruction::Mul => todo!(),
            | Instruction::Div => todo!(),
            | Instruction::Jump => todo!(),
            | Instruction::JumpIf => todo!(),
            | Instruction::Push => todo!(),
            | Instruction::Pop => todo!(),
            | Instruction::Compare => todo!(),
            | Instruction::Increment => todo!(),
            | Instruction::Decrement => todo!(),
            | Instruction::DebugRead => todo!(),
            | Instruction::DebugDumpReg => todo!(),
            | Instruction::EnumLength => todo!(),
        }
    }
}
