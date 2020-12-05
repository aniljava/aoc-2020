use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub fn file_to_lines(path: &str) -> Vec<String> {
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut str_lines: Vec<String> = vec![];

    for line in lines {
        if let Ok(line) = line {
            str_lines.push(line);
        };
    }
    str_lines
}
