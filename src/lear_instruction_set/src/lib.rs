#![doc = include_str!("../README.md")]

use crate::instruction_set::{Instruction, InstructionSet};

pub mod instruction_set;

/// Function to initialise the Instruction set in an easily usable manner.
///
/// # Example
/// ```
/// use lear_instruction_set::init_instruction_set;
/// use crate::lear_instruction_set;
///
/// let instruction_set = init_instruction_set() ;
/// ```
///
pub fn init_instruction_set() -> InstructionSet {
    let mut initial_set = instruction_set::InstructionSet::new();
    initial_set.add_instruction(Instruction::new(0x20, "ADD", 1));
    initial_set.add_instruction(Instruction::new(0x21, "SUB", 1));
    initial_set.add_instruction(Instruction::new(0x10, "STA", 1));
    initial_set.add_instruction(Instruction::new(0x15, "STI", 1));
    initial_set.add_instruction(Instruction::new(0x11, "LDA", 1));
    initial_set.add_instruction(Instruction::new(0x16, "LDI", 1));
    initial_set.add_instruction(Instruction::new(0x01, "JMP", 1));
    initial_set.add_instruction(Instruction::new(0x02, "JMZ", 1));
    initial_set.add_instruction(Instruction::new(0x1F, "DAT", 2));
    initial_set.add_instruction(Instruction::new(0x0, "NOP", 0));
    initial_set.add_instruction(Instruction::new(0x30, "PRN", 0));
    initial_set.add_instruction(Instruction::new(0xFFFFFFFFFFFFFFFF, "HLT", 0));

    initial_set.clone()
}
