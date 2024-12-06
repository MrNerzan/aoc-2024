use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = load_file();
    let mut result_p1 = 0;
    let mut result_p2 = 0;
    let mut found_linebreak = false;
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    // part 1
    for line in lines {
        if !found_linebreak && line.is_empty() {
            found_linebreak = true;
        } else if found_linebreak {
            updates.push(line);
        } else {
            rules.push(line);
        }
    }

    let mut first = Vec::new();
    let mut after = Vec::new();

    match build_lists(rules) {
        Ok((f, s)) => {
            first = f;
            after = s;
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }

    let mut order_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (f, a) in first.iter().zip(after.iter()) {
        order_map.entry(*f).or_insert_with(HashSet::new).insert(*a);
    }
    updates
        .iter_mut()
        .enumerate()
        .for_each(|(_index, update_str)| {
            let mut fixed = false;
            let mut update: Vec<i32> = update_str
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();

            let mut valid = true;
            let mut positions = HashMap::new();

            for (i, &page) in update.iter().enumerate() {
                positions.insert(page, i);
            }

            loop {
                // Jesus this is fugly
                let mut modified = false;

                for (before, after_pages) in &order_map {
                    if positions.contains_key(before) {
                        for &after in after_pages {
                            if positions.contains_key(&after) {
                                if positions[before] > positions[&after] {
                                    fixed = true;
                                    let flipped = flip_elements(update_str, before, after);
                                    *update_str = flipped;
                                    modified = true;
                                    valid = false;
                                    break;
                                }
                            }
                        }
                    }
                    if !valid {
                        break;
                    }
                }
                if !modified {
                    break;
                }

                update = update_str
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                positions.clear();
                for (i, &page) in update.iter().enumerate() {
                    positions.insert(page, i);
                }

                valid = true;
            }
            if valid {
                if fixed {
                    result_p2 += update[update.len() / 2];
                } else {
                    result_p1 += update[update.len() / 2];
                }
            }
        });
    println!("Part 1: {}", result_p1);
    // part 2
    println!("Part 2: {}", result_p2);
}

fn load_file() -> Vec<String> {
    let path = "day_5.txt";
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
            .split_once("|")
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

fn flip_elements(update_str: &mut String, before: &i32, after: i32) -> String {
    let mut elements: Vec<String> = update_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    if let (Some(before_index), Some(after_index)) = (
        elements
            .iter()
            .position(|x| x.parse::<i32>().ok() == Some(*before)),
        elements
            .iter()
            .position(|x| x.parse::<i32>().ok() == Some(after)),
    ) {
        elements.swap(before_index, after_index);
    }

    elements.join(",")
}
