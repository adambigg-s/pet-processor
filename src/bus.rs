#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BusState {
    #[default]
    Idle,
    Busy,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BusInstruction {
    #[default]
    Null,
    Read,
    Write,
}

#[derive(Debug, Default)]
pub struct Bus<Address, Data> {
    state: BusState,
    instruction: BusInstruction,
    address: Option<Address>,
    data: Option<Data>,
}

impl<Address, Data> Bus<Address, Data> {
    pub fn is_avaliable(&self) -> bool {
        self.state == BusState::Idle && self.instruction == BusInstruction::Null
    }

    pub fn get_instruction(&self) -> BusInstruction {
        self.instruction
    }

    pub fn read_data(&mut self) -> Option<Data> {
        self.data.take()
    }

    pub fn read_address(&mut self) -> Option<Address> {
        self.address.take()
    }

    pub fn dispatch_read(&mut self, address: Address) -> Option<()> {
        if !self.is_avaliable() {
            return None;
        }

        self.instruction = BusInstruction::Read;
        self.state = BusState::Busy;
        self.address = Some(address);
        Some(())
    }

    pub fn complete_read(&mut self, data: Data) {
        self.data = Some(data);
        self.state = BusState::Idle;
        self.instruction = BusInstruction::Null;
    }

    pub fn dispatch_write(&mut self, address: Address, data: Data) -> Option<()> {
        if !self.is_avaliable() {
            return None;
        }

        self.instruction = BusInstruction::Null;
        self.state = BusState::Busy;
        self.address = Some(address);
        self.data = Some(data);
        Some(())
    }

    pub fn complete_write(&mut self) {
        self.state = BusState::Idle;
        self.instruction = BusInstruction::Null;
    }
}
