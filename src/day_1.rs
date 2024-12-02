use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file();
}

fn load_file() -> Vec<String> {
    let path = "input.txt";
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}
