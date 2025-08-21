use crate::memory::Addressable;

pub struct ProgramAssembler<'d, Memory> {
    head: usize,
    memory: &'d mut Memory,
}

impl<'d, Memory, Data> ProgramAssembler<'d, Memory>
where
    Memory: Addressable<usize, Data = Data>,
{
    pub fn build(target: &'d mut Memory) -> Self {
        Self { head: Default::default(), memory: target }
    }

    pub fn assemble_program<Instructions>(&mut self, program: Vec<Instructions>)
    where
        Instructions: IntoIterator<Item = Data>,
    {
        for line in program {
            self.assemble_instruction(line);
        }
    }

    fn assemble_instruction<Instructions>(&mut self, instructions: Instructions)
    where
        Instructions: IntoIterator<Item = Data>,
    {
        for instruction in instructions {
            *self.memory.write(self.head) = instruction;
            self.head += 1;
        }
    }
}
