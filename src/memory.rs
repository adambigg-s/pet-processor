pub trait Addressable<Address> {
    type Data;

    fn read(&self, address: Address) -> Self::Data;

    fn write(&mut self, address: Address) -> &mut Self::Data;
}

#[derive(Debug)]
pub struct MemoryBlock<const M: usize, Data> {
    memory: [Data; M],
}

impl<const M: usize, Data> Default for MemoryBlock<M, Data>
where
    Data: Default + Copy,
{
    fn default() -> Self {
        Self { memory: [Default::default(); M] }
    }
}

impl<const M: usize, Data, Address> Addressable<Address> for MemoryBlock<M, Data>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_read() {
        let mut mem = MemoryBlock::<8, u8>::default();
        mem.memory[5] = 33;
        assert!(mem.read(5u16) == 33);
    }

    #[test]
    fn mem_write() {
        let mut mem = MemoryBlock::<8, u8>::default();
        *mem.write(5u16) = 33;
        assert!(mem.memory[5] == 33);
    }
}
