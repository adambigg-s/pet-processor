use std::fmt::Debug;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBarStruct;

trait _FooBarTrait<T>
where
    T: Debug + Default + Clone + Copy + Sized,
{
}

const RAM_SIZE: usize = 32;

fn main() {
    let mut processor = Processor {};
    let mut ram = MemoryBlock::<RAM_SIZE, u8>::default();
    let mut clock = Clock::default();

    dbg!(&ram);
    dbg!(&processor);

    processor_run(&mut processor, &mut ram, &mut clock);
}

fn processor_run<const N: usize, Data>(
    cpu: &mut Processor,
    ram: &mut MemoryBlock<N, Data>,
    clock: &mut Clock,
) {
    loop {
        clock.tick += 1;
    }
}

#[derive(Debug, Default)]
struct Clock {
    tick: usize,
}

#[derive(Debug)]
struct Processor {}

#[derive(Debug)]
struct MemoryBlock<const N: usize, Memory> {
    memory: [Memory; N],
}

impl<const N: usize, Data> Default for MemoryBlock<N, Data>
where
    Data: Default + Copy,
{
    fn default() -> Self {
        Self { memory: [Default::default(); N] }
    }
}

impl<const N: usize, Data, Address> Addressable<Address> for MemoryBlock<N, Data>
where
    Address: Into<usize>,
    Data: Copy,
{
    type Data = Data;

    fn read(&self, address: Address) -> Self::Data {
        self.memory[address.into()]
    }

    fn write(&mut self, address: Address) -> &mut Self::Data {
        &mut self.memory[address.into()]
    }
}

trait Addressable<Address> {
    type Data;

    fn read(&self, address: Address) -> Self::Data;

    fn write(&mut self, address: Address) -> &mut Self::Data;
}

#[cfg(test)]
mod tests {
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
}
