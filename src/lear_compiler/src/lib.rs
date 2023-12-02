#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;
use lear_instruction_set::init_instruction_set;


/// Extracts the portion of the input string up to the first occurrence of a delimiter.
///
/// # Arguments
///
/// * `input` - The input string from which to extract the portion.
///
/// # Returns
///
/// The portion of the input string up to the first occurrence of a delimiter. If no delimiter is found,
/// an empty string is returned.
///
fn up_to_delim(input: &str) -> &str {
    let delimiters = ['\n', ';'];
    let splits: Vec<&str> = input.split(|c| delimiters.contains(&c)).collect();
    splits.get(0).unwrap_or(&"")
}

/// Finds tokens in the input string by matching a regular expression pattern.
///
/// # Arguments
///
/// * `input` - The input string to search for tokens.
///
/// # Returns
///
/// A vector of strings containing the found tokens.
///
fn find_tokens(input: &str) -> Vec<String> {
    let re = Regex::new(r"(?s)\w+:\s*\{.*?\}").unwrap();
    let mat: Vec<String> = re.captures_iter(input)
        .map(|cap| {
            let token = cap.get(0)
                .map_or("", |m| m.as_str());

            // Apply up_to_delim to every line within the token
            let lines: Vec<&str> = token.split('\n').collect();
            let processed_lines: Vec<String> = lines.into_iter()
                .map(|line| up_to_delim(line).to_string())
                .collect();

            // Join back processed lines into a single string
            let processed_token = processed_lines.join("\n");
            processed_token
        }).collect();
    mat
}

fn compile_token(token: &str) -> Option<Vec<(String, Vec<&str>)>> {
    // Get the instruction set
    let ins_set = init_instruction_set();

    // Split the token into separate lines
    let mut lines: Vec<&str> = token.lines().collect();

    // Ignore the first and last line
    if lines.len() > 2 {
        lines = lines[1..lines.len()-1].to_vec();
    } else {
        return None;
    }

    // Process each line separately
    let mut instructions = Vec::new();
    for line in lines {
        // Split the line into instruction and arguments
        let parts: Vec<&str> = line.split_whitespace().collect();
        let (inst_name, args) = parts.split_at(1);

        // Check that the instruction name matches an instruction in the InstructionSet,
        // and that the number of arguments matches the number expected for that instruction
        match ins_set.get_instruction_by_name((*inst_name[0]).to_string()) {
            Some(instruction) => {
                if instruction.get_num_arg() == (args.len() as u8) {
                    // Convert the instruction encoding to a hexadecimal string
                    let encoding_str = format!("0x{:02X}", instruction.get_encoding());

                    // If everything is fine, push the stringified instruction and arguments to the list
                    instructions.push((encoding_str, args.to_vec()));
                } else {
                    println!("Incorrect number of arguments for instruction.");
                    return None;
                }
            },
            None => return None,
        }
    }
    Some(instructions)
}

pub fn compile(path_string: String) {
    let source_path = Path::new(path_string.as_str());
    let dis = source_path.display();
    let mut file = match File::open(&source_path) {
        Ok(file) => file,
        Err(cause) => panic!("Couldn't open {}: {}", dis, cause),
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => println!("File content read: {}", data),
        Err(cause) => panic!("Couldn't read {}: {}", dis, cause),
    };

    let tokens = find_tokens(&data);

    let compiledTokens: Vec<&str>;

}