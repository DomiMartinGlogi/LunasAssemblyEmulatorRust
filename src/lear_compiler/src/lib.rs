#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;


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
}