use crate::bus::Bus;
use crate::bus::BusState;
use crate::cpu::Data;
use crate::cpu::Pointer;

pub trait Addressable<Address> {
    type Data;

    fn read(&self, address: Address) -> Self::Data;

    fn write(&mut self, address: Address) -> &mut Self::Data;
}

pub struct MemoryBlock<const M: usize, Data> {
    memory: [Data; M],
}

impl<const M: usize, Data> std::fmt::Debug for MemoryBlock<M, Data>
where
    Data: std::fmt::Debug,
{
    fn fmt(&self, frmtr: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(frmtr, "[")?;
        for chunk in self.memory.chunks(32) {
            write!(frmtr, "    ")?;
            for item in chunk.iter() {
                write!(frmtr, "{item:2?}, ")?;
            }
            writeln!(frmtr)?;
        }
        write!(frmtr, "]")
    }
}

impl<const M: usize, Data> std::fmt::Display for MemoryBlock<M, Data>
where
    Data: std::fmt::Debug,
{
    fn fmt(&self, frmtr: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(frmtr, "[")?;
        for item in &self.memory {
            write!(frmtr, "{item:2?}, ")?;
        }
        write!(frmtr, "]")
    }
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

pub fn memory_cycle<const M: usize>(ram: &mut MemoryBlock<M, Data>, bus: &mut Bus<Pointer, Data>) {
    match bus.get_instruction() {
        | BusState::Read => {
            let bus_state = bus.complete_dispatch();
            assert!(bus_state.address.is_some());
            assert!(bus_state.data.is_none());
            if let Some(address) = bus_state.address.take() {
                let read_back = ram.read(address);
                *bus_state.data = Some(read_back);
            }
        }
        | BusState::Write => {
            let bus_state = bus.complete_dispatch();
            assert!(bus_state.address.is_some());
            assert!(bus_state.data.is_some());
            if let (Some(address), Some(data)) = (bus_state.address.take(), bus_state.data.take()) {
                *ram.write(address) = data;
            }
        }
        | BusState::Null => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_read() {
        let mut mem = MemoryBlock::<8, u8>::default();
        mem.memory[5] = 33;
        assert!(mem.read(5_u16) == 33);
    }

    #[test]
    fn mem_write() {
        let mut mem = MemoryBlock::<8, u8>::default();
        *mem.write(5_u16) = 33;
        assert!(mem.memory[5] == 33);
    }
}
