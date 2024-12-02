use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file();

    let list = build_list(lines);

    let (safe_reports, damped_reports): (i32, i32) =
        list.iter().fold((0, 0), |(safe, damped), row| {
            let (safe_row, damped_safe) = is_safe(row.clone());
            (safe + safe_row as i32, damped + damped_safe as i32)
        });

    println!(
        "Reports -> (Safe: {:?}, Damped: {:?})",
        safe_reports, damped_reports
    );
    println!("Total Safe: {:?}", safe_reports + damped_reports);
}

fn load_file() -> Vec<String> {
    let path = "day_2.txt";
    let file = File::open(&path).expect("Could not open file");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}

fn build_list(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut arr = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
        arr.push(numbers);
    }
    arr
}

fn is_safe(row: Vec<i32>) -> (bool, bool) {
    let (safe_by_diff, safe_by_dir) = is_row_safe(&row);
    let all_safe = safe_by_diff && safe_by_dir;

    if !all_safe {
        let damped_safe = (0..row.len()).any(|i| {
            let mut modified_row = row.clone();
            modified_row.remove(i);
            let (safe_diff, safe_dir) = is_row_safe(&modified_row);
            safe_diff && safe_dir
        });

        return (false, damped_safe);
    }
    (true, false)
}

fn is_row_safe(row: &[i32]) -> (bool, bool) {
    let mut safe_by_diff = true;
    let mut safe_by_dir = true;
    let mut dir = if row.len() > 1 {
        row[1].cmp(&row[0]) as i32
    } else {
        0
    };

    for i in 1..row.len() {
        let diff = (row[i - 1] - row[i]).abs();
        if diff < 1 || diff > 3 {
            safe_by_diff = false;
            break;
        }

        match dir {
            1 if row[i] < row[i - 1] => {
                safe_by_dir = false;
                break;
            }
            -1 if row[i] > row[i - 1] => {
                safe_by_dir = false;
                break;
            }
            0 => {
                dir = row[i].cmp(&row[i - 1]) as i32;
            }
            _ => {}
        }
    }

    (safe_by_diff, safe_by_dir)
}
