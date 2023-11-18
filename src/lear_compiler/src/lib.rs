#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn compile(path_string: String) {
    let source_path = Path::new(path_string.as_str());
    let dis = source_path.display();
    let mut file = match File::open(&source_path) {
        Ok(file) => {file}
        Err(cause) => {panic!("Couldn't open {}: {}", dis, cause)}
    };



}