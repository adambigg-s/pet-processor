use crate::memory::Addressable;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::memory::MemoryBlock;

    use super::*;

    use std::fmt::Debug;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct _FooBarStruct;

    trait _FooBarTrait<T>
    where
        T: Debug + Default + Clone + Copy + Sized,
    {
    }

    #[test]
    fn label() {
        let mut mem = MemoryBlock::<10, u8>::default();
        let writer = ProgramAssembler::build(&mut mem);
        todo!("need to implement a way to store symbolic labels for jumps {:?}", writer);
    }
}
