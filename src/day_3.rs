use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file();

    let mut total_sum: i32 = 0;

    // part 1

    for line in 0..lines.len() {
        total_sum += sum_of_multiplications_in_line(&lines[line]);
    }
    println!("Part 1: {:?}", total_sum);

    // part 2
    total_sum = 0;

    for line in &lines {
        let mut in_ignored_section = false;

        let mut processed_line = String::new();
        for (idx, ch) in line.chars().enumerate() {
            if line[idx..].starts_with("don't()") {
                in_ignored_section = true;
                continue;
            }

            if line[idx..].starts_with("do()") {
                in_ignored_section = false;
                continue;
            }

            if !in_ignored_section {
                processed_line.push(ch);
            }
        }

        total_sum += sum_of_multiplications_in_line(&processed_line);
    }

    println!("Part 2: {:?}", total_sum);
}

fn load_file() -> Vec<String> {
    let path = "day_3.txt";
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

fn sum_of_multiplications_in_line(processed_line: &str) -> i32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    re.captures_iter(&processed_line)
        .map(|cap| {
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();
            num1 * num2
        })
        .sum()
}
