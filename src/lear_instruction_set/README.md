# lear_instruction_set

## Instructions

### Instruction Architecture

> 1 Word is assumed to be 64 Bit

Instructions are all 1 Word and can carry up to 64 Arguments of 1 Word Length

### Instruction Set

| Instruction | Encoding             | Arguments                             | Usage                                                                           |
|-------------|----------------------|---------------------------------------|---------------------------------------------------------------------------------|
| `ADD`       | `0x20`               | `x` - Load Address                    | Adds the Value at address `x` to the Accumulator Register                       |
| `SUB`       | `0x21`               | `x` - Load Address                    | Subtracts the Value at address `x` from the Accumulator Register                |
| `STA`       | `0x10`               | `x` - Write Address                   | Writes the Value in the Accumulator to address `x`                              |
| `STI`       | `0x15`               | `x` - Load Address                    | Reads the Value at `x` and interprets it as a pointer to the write address      |
| `LDA`       | `0x11`               | `x` - Load Address                    | Loads the Value at `x` into the Accumulator                                     |
| `LDI`       | `0x16`               | `x` - Load Address                    | Load the value at the address stored at memory address `x` into the accumulator |
| `JMP`       | `0x01`               | `x` - Target Address                  | Sets the Instruction Pointer to `x`                                             |
| `JMZ`       | `0x02`               | `x` - Target Address                  | Sets the Instruction Pointer to `x` if the Accumulator is 0                     |
| `HLT`       | `0xFFFFFFFFFFFFFFFF` | -                                     | Stops Execution                                                                 |
| `DAT`       | `0x1F`               | `x` - Target Address <br> `y` - Value | Sets the Data Memory Address `x` to contain `y`                                 |
| `NOP`       | `0x0`                | -                                     | Halts execution for 1 Cycle                                                     |
| `PRN`       | `0x30`               | -                                     | Prints the Value in the Accumulator to the Standard Output                      |