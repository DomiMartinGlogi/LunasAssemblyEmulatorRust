/// Struct representing an Instruction
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
    /// let ins = Instruction::new(0x00, "NOP", 0);
    /// ```
    pub fn new(encoding: u64, name: &str, num_arg: u8) -> Instruction{
        Instruction{
            encoding,
            name: name.to_string(),
            num_arg,
        }
    }
}

