use crate::CYCLE_LIMIT;
use crate::RAM_SIZE;
use crate::bus::Bus;
use crate::bus::Cycle;
use crate::clock::Clock;
use crate::instructions;
use crate::instructions::Instruction;
use crate::memory::Addressable;
use crate::memory::MemoryBlock;

pub type Data = u8;
pub type Pointer = u16;

type RegisterArray<const R: usize, Data> = MemoryBlock<R, Data>;

#[derive(Debug, Default)]
pub struct ProcFlags {
    pub zero: bool,
    pub less: bool,
    pub great: bool,
    pub complete: bool,
}

impl ProcFlags {
    pub fn reset_complete(&mut self) {
        self.complete = Default::default();
    }

    pub fn reset_logical(&mut self) {
        self.zero = Default::default();
        self.less = Default::default();
        self.great = Default::default();
    }
}

#[derive(Debug, Default)]
pub struct OperandBuffer<const N: usize, Data>
where
    Data: Copy,
{
    operands: MemoryBlock<N, Option<Data>>,
    required: usize,
    fetched: usize,
    reader_head: usize,
}

impl<const N: usize, Data> OperandBuffer<N, Data>
where
    Data: Default + Copy,
{
    pub fn is_full(&mut self) -> bool {
        self.fetched == self.required
    }

    pub fn push(&mut self, operand: Data) {
        *self.operands.write(self.fetched) = Some(operand);
        self.fetched += 1;
    }

    pub fn read_next(&mut self) -> Data {
        let out = self.operands.read(self.reader_head % self.fetched);
        self.reader_head += 1;
        out.expect("uninitialized operand - buffer too short")
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Default)]
pub enum ProcState {
    #[default]
    Idle,
    FetchInit,
    Decode,
    FetchOperands,
    Execute,
    WriteBack,
}

#[derive(Debug, Default)]
pub struct MicroState(pub Data);

impl MicroState {
    pub fn reset(&mut self) {
        let Self(val) = self;
        *val = Default::default();
    }

    pub fn increment(&mut self) {
        let Self(val) = self;
        *val += 1;
    }
}

fn procstate_idle<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    cpu.initiate_fetch(bus);
    cpu.state = ProcState::FetchInit;
}

fn procstate_fetch_init<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    let Some(instruction) = bus.read_data()
    else {
        return;
    };

    cpu.current_instruction = instruction.into();
    cpu.state = ProcState::Decode;
}

fn procstate_decode<const R: usize>(cpu: &mut Processor<R>) {
    cpu.operand_buffer.reset();
    cpu.microstate.reset();
    cpu.flags.reset_complete();
    cpu.operand_buffer.required = Instruction::operand_count(&cpu.current_instruction);
    cpu.state = ProcState::FetchOperands;
}

fn procstate_fetch_operands<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    if let Some(operand) = bus.read_data() {
        cpu.operand_buffer.push(operand);
    }

    if !cpu.operand_buffer.is_full() {
        cpu.initiate_fetch(bus);
        return;
    }
    cpu.state = ProcState::Execute;
}

fn procstate_execute<const R: usize>(cpu: &mut Processor<R>, bus: &mut Bus<Pointer, Data>) {
    if !cpu.flags.complete {
        cpu.execute(bus);
        return;
    }

    cpu.state = ProcState::WriteBack;
}

fn procstate_writeback<const R: usize>(cpu: &mut Processor<R>) {
    cpu.current_instruction = Instruction::Null;
    cpu.state = ProcState::Idle;
}

#[derive(Debug)]
pub struct Processor<const R: usize> {
    pub program_counter: Pointer,
    pub stack_pointer: Pointer,
    pub registers: RegisterArray<R, Data>,
    pub flags: ProcFlags,
    pub halted: bool,
    pub state: ProcState,
    pub microstate: MicroState,
    pub current_instruction: Instruction,
    pub operand_buffer: OperandBuffer<R, Data>,
}

impl<const R: usize> Default for Processor<R> {
    fn default() -> Self {
        Self {
            program_counter: Default::default(),
            stack_pointer: (RAM_SIZE - 1) as Pointer,
            registers: Default::default(),
            flags: Default::default(),
            halted: Default::default(),
            state: Default::default(),
            microstate: MicroState::default(),
            current_instruction: Default::default(),
            operand_buffer: Default::default(),
        }
    }
}

pub fn _processor_run_debug<const M: usize, const R: usize>(
    cpu: &mut Processor<R>,
    ram: &mut MemoryBlock<M, Data>,
    bus: &mut Bus<Pointer, Data>,
    clock: &mut Clock,
) {
    while !cpu.halted && clock.tick < CYCLE_LIMIT {
        let ms_wait = 25;

        println!("\x1b[2J\x1b[0H{:?}\n{:?}\n{:?}\n{:?}", &ram, &cpu, &bus, &clock);
        std::thread::sleep(std::time::Duration::from_millis(ms_wait));

        ram.cycle(bus);

        println!("\x1b[2J\x1b[0H{:?}\n{:?}\n{:?}\n{:?}", &ram, &cpu, &bus, &clock);
        std::thread::sleep(std::time::Duration::from_millis(ms_wait));

        cpu.cycle(bus);

        clock.tick += 1;
    }
}

pub fn processor_run<const M: usize, const R: usize>(
    cpu: &mut Processor<R>,
    ram: &mut MemoryBlock<M, Data>,
    bus: &mut Bus<Pointer, Data>,
    clock: &mut Clock,
) {
    while !cpu.halted && clock.tick < CYCLE_LIMIT {
        cpu.cycle(bus);
        ram.cycle(bus);
        clock.tick += 1;
    }
}

impl<const R: usize> Cycle<Pointer, Data> for Processor<R> {
    fn cycle(&mut self, bus: &mut Bus<Pointer, Data>) {
        match self.state {
            | ProcState::Idle => procstate_idle(self, bus),
            | ProcState::FetchInit => procstate_fetch_init(self, bus),
            | ProcState::Decode => procstate_decode(self),
            | ProcState::FetchOperands => procstate_fetch_operands(self, bus),
            | ProcState::Execute => procstate_execute(self, bus),
            | ProcState::WriteBack => procstate_writeback(self),
        }
    }
}

impl<const R: usize> Processor<R> {
    fn initiate_fetch(&mut self, bus: &mut Bus<Pointer, Data>) {
        assert!(bus.is_avaliable());
        bus.dispatch_read(self.program_counter);
        self.program_counter += 1;
    }

    fn execute(&mut self, bus: &mut Bus<Pointer, Data>) {
        match self.current_instruction {
            | Instruction::Halt => instructions::halt(self),
            | Instruction::Ret => instructions::ret(self, bus),
            | Instruction::LoadImm => instructions::load_imm(self),
            | Instruction::LoadMem => instructions::load_mem(self, bus),
            | Instruction::Copy => instructions::copy(self),
            | Instruction::Add => instructions::add(self),
            | Instruction::Sub => instructions::sub(self),
            | Instruction::Mul => instructions::mul(self),
            | Instruction::Div => instructions::div(self),
            | Instruction::Jump => instructions::jump(self),
            | Instruction::JumpIfZero => instructions::jump_if_zero(self),
            | Instruction::Push => instructions::push(self, bus),
            | Instruction::Pop => instructions::pop(self, bus),
            | Instruction::Compare => instructions::compare(self),
            | Instruction::Increment => instructions::increment(self),
            | Instruction::Decrement => instructions::decrement(self),
            | Instruction::Null => {}
            | Instruction::EnumLength => panic!("this can only be explained by corrupt bytes"),
        }
    }
}
