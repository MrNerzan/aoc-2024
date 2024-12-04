use std::fs::File;
use std::io::{self, BufRead};

fn load_file() -> Vec<String> {
    let path = "input.txt";
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

fn build_lists(lines: Vec<String>) -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut first = Vec::new();
    let mut second = Vec::new();

    for line in lines {
        let parts = line
            .split_once("   ")
            .ok_or(format!("Invalid format: {}", line))?;
        let part1 = parts
            .0
            .trim()
            .parse::<i32>()
            .map_err(|_| format!("Invalid number: {}", parts.0))?;
        let part2 = parts
            .1
            .trim()
            .parse::<i32>()
            .map_err(|_| format!("Invalid number: {}", parts.1))?;
        first.push(part1);
        second.push(part2);
    }

    Ok((first, second))
}

fn main() {
    let lines = load_file();

    let (mut arr1, mut arr2) = match build_lists(lines) {
        Ok((arr1, arr2)) => (arr1, arr2),
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // part 1
    arr1.sort();
    arr2.sort();

    let dist: i32 = (0..arr1.len())
        .map(|i| (arr1[i] - arr2[i]).abs() as i32)
        .sum();

    println!("Dist: {:?}", dist);

    // part 2
    let score: usize = (0..arr1.len())
        .map(|i| {
            let freq = arr2.iter().filter(|&x| *x == arr1[i]).count();
            freq * arr1[i] as usize
        })
        .sum();

    println!("Score: {:?}", score);

    // single opereation

    let (dist, score): (i32, usize) = (0..arr1.len())
        .map(|i| {
            let diff = (arr1[i] - arr2[i]).abs() as i32;
            let count = arr2.iter().filter(|&&x| x == arr1[i]).count() * arr1[i] as usize;
            (diff, count)
        })
        .fold((0, 0), |(cum_dist, cum_score), (diff, count)| {
            (cum_dist + diff, cum_score + count)
        });

    println!("Dist: {:?}", dist);
    println!("Score: {:?}", score);
}
