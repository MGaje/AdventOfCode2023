use std::fs::read_to_string;

pub fn read_file_to_lines(file: &str) -> Vec<String> {
    return read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}