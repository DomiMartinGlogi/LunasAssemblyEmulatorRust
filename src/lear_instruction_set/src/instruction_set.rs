/// Struct representing an Instruction
#[derive(Clone)]
pub struct Instruction {
    /// 64-Bit encoding of the Instruction
    encoding: u64,
    /// Name/Keyword of the Instruction
    name: String,
    /// Number of Arguments of the Instruction
    num_arg: u8
}

impl Instruction {
    /// Returns an Instruction
    ///
    /// # Arguments
    ///
    /// * `encoding` - u64 CPU Encoding
    /// * `name` - Name/Keyword of the Instruction
    /// * `num_arg` - Number of Arguments the Instruction takes
    ///
    /// # Example
    /// ```
    /// use lear_instruction_set::instruction_set::Instruction;
    ///
    /// let ins = Instruction::new(0x00, "NOP", 0);
    /// ```
    pub fn new(encoding: u64, name: &str, num_arg: u8) -> Instruction {
        Instruction{
            encoding,
            name: name.to_string(),
            num_arg,
        }
    }

    /// Method to return the 64 Bit encoding of the Instruction
    pub fn get_encoding(&self) -> u64 {
        self.encoding.clone()
    }

    /// Method to return the name of the Instruction
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Method to return the number of Arguments that the Instruction takes
    pub fn get_num_arg(&self) -> u8 {
        self.num_arg.clone()
    }

}

#[derive(Clone)]
pub struct InstructionSet {
    instructions: Vec<Instruction>,
    instruction_number: u64,
}

impl InstructionSet {

    /// Returns an Empty Instruction Set
    ///
    /// # Arguments
    ///
    /// This function purposefully does not take any arguments
    ///
    /// # Example
    /// ```
    /// use lear_instruction_set::instruction_set::InstructionSet;
    ///
    /// let insSet = InstructionSet::new();
    /// ```
    pub fn new() -> InstructionSet {
        InstructionSet{
            instructions: Vec::new(),
            instruction_number: 0,
        }
    }

    /// Returns an Option containing the Instruction if an Instruction of a given name exists
    ///
    /// # Arguments
    ///
    /// * `name` - A boxed string slice (`Box<str>`) containing the name of the instruction to find. This method uses `Box<str>` for potential memory benefits when dealing with large amounts of strings.
    ///
    /// # Example
    /// ```
    /// use lear_instruction_set::instruction_set::InstructionSet;
    ///
    /// let insSet = InstructionSet::new();
    /// let insOpt = insSet.get_instruction_by_name("NOP".to_string());
    /// ```
    pub fn get_instruction_by_name(&self, name: String) -> Option<Instruction> {
        for instruction in &self.instructions {
            if instruction.name.as_str() == &*name {
                return Some(instruction.clone());
            }
        }
        None
    }

    pub fn add_instruction(&mut self, ins: Instruction) -> bool {
        match self.get_instruction_by_name(ins.get_name()) {
            None => {self.instructions.push(ins); self.instruction_number += 1; return true}
            Some(_) => {return false}
        }
    }
}
