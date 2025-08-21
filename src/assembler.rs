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

// assembler.assemble_program(vec![
//     vec![Instruction::LoadImm.into(), 3, 10],
//     vec![Instruction::LoadImm.into(), 1, 0],
//     vec![Instruction::LoadImm.into(), 2, 1],
//     vec![Instruction::LoadImm.into(), 4, 0],
//     vec![Instruction::Push.into(), 1],
//     vec![Instruction::Push.into(), 2],
//     vec![Instruction::Add.into(), 0, 1, 2],
//     vec![Instruction::Push.into(), 0],
//     vec![Instruction::Copy.into(), 1, 2],
//     vec![Instruction::Copy.into(), 2, 0],
//     vec![Instruction::Decrement.into(), 3],
//     vec![Instruction::Compare.into(), 3, 4],
//     vec![Instruction::JumpIf.into(), 16],
//     vec![Instruction::Halt.into()],
// ]);

// enum AsmTok<T> {
//     Label(&'static str),
//     Ins(T),
// }

//     fn assemble_instruction<Tokens>(&mut self, instructions: Tokens)
//     where
//         Tokens: IntoIterator<Item = AsmTok<Data>>,
//     {
//         for token in instructions {
//             let instruction = match token {
//                 | AsmTok::Label(label) => {
//                     *self.labels.get(label).expect("assembler failed - no archived label")
//                 }
//                 | AsmTok::Ins(ins) => ins,
//             };

//             *self.memory.write(self.head) = instruction;
//             self.head += 1;
//         }
//     }

//     fn archive_labels<Token>(&mut self, program: &Vec<Token>)
//     where
//         Token: IntoIterator<Item = AsmTok<Data>>,
//     {
//         let mut index = 0;
//         for &line in program {
//             for token in line {
//                 if let AsmTok::Label(label) = token {
//                     self.labels.insert(label, index);
//                 }
//                 else {
//                     index += 1;
//                 }
//             }
//         }
//     }

//     pub fn assemble_program<Token>(&mut self, program: Vec<Token>)
//     where
//         Token: IntoIterator<Item = AsmTok<Data>>,
//     {
//         self.archive_labels(&program);

//         for line in program {
//             self.assemble_instruction(line);
//         }
//     }
