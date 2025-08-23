pub struct BusResponse<'d, Address, Data> {
    pub address: &'d mut Option<Address>,
    pub data: &'d mut Option<Data>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BusState {
    #[default]
    Null,
    Read,
    Write,
}

#[derive(Debug, Default)]
pub struct Bus<Address, Data> {
    instruction: BusState,
    address: Option<Address>,
    data: Option<Data>,
}

impl<Address, Data> Bus<Address, Data> {
    pub fn is_avaliable(&self) -> bool {
        assert!(self.address.is_none());
        assert!(self.data.is_none());
        self.instruction == BusState::Null
    }

    pub fn get_instruction(&self) -> BusState {
        self.instruction
    }

    pub fn read_data(&mut self) -> Option<Data> {
        self.data.take()
    }

    pub fn dispatch_read(&mut self, address: Address) -> Option<()> {
        assert!(self.is_avaliable());
        if !self.is_avaliable() {
            return None;
        }

        self.instruction = BusState::Read;
        self.address = Some(address);
        Some(())
    }

    pub fn dispatch_write(&mut self, address: Address, data: Data) -> Option<()> {
        assert!(self.is_avaliable());
        if !self.is_avaliable() {
            return None;
        }

        self.instruction = BusState::Write;
        self.address = Some(address);
        self.data = Some(data);
        Some(())
    }

    pub fn complete_dispatch(&mut self) -> BusResponse<Address, Data> {
        self.instruction = BusState::Null;
        BusResponse { address: &mut self.address, data: &mut self.data }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::{Addressable, MemoryBlock, memory_cycle};

    use super::*;

    #[test]
    fn reader_dispatch() {
        let mut ram = MemoryBlock::<8, u8>::default();
        let mut bus = Bus::<u8, u8>::default();
        *ram.write(3_u8) = 33;
        bus.dispatch_read(3);
        memory_cycle(&mut ram, &mut bus);
        assert!(bus.instruction == BusState::Null);
        assert!(bus.data == Some(33));
    }

    #[test]
    fn writer_dispatch() {
        let mut ram = MemoryBlock::<8, u8>::default();
        let mut bus = Bus::<u8, u8>::default();
        bus.dispatch_write(3, 33);
        memory_cycle(&mut ram, &mut bus);
        assert!(bus.instruction == BusState::Null);
        assert!(ram.read(3_u8) == 33);
    }
}
