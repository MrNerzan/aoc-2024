use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file();
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    // part 1
    println!("Part 1: {}", result_p1);
    // part 2
    println!("Part 2: {}", result_p2);
}

fn load_file() -> Vec<String> {
    let path = "day_6.txt";
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}
