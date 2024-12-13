use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn is_safe_row(numbers: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];

        if diff < 1 || diff > 3 {
            increasing = false;
        }
        if diff > -1 || diff < -3 {
            decreasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

fn read_data_from_txt<P: AsRef<Path>>(filepath: P) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<Result<_, _>>()?;
        rows.push(numbers);
    }

    Ok(rows)
}

pub fn part1(data: &Vec<Vec<i32>>) {
    let mut safe_row_count = 0;

    for row in data {
        if is_safe_row(row) {
            safe_row_count += 1;
        }
    }

    println!("Part 1 - Number of safe rows: {}", safe_row_count);
}

pub fn part2(data: &Vec<Vec<i32>>) {
    let mut safe_row_count = 0;

    for row in data {
        if is_safe_row(row) {
            safe_row_count += 1;
        } else {
            let mut made_safe = false;
            for i in 0..row.len() {
                let mut temp_row = row.clone();
                temp_row.remove(i);
                if is_safe_row(&temp_row) {
                    made_safe = true;
                    break;
                }
            }
            if made_safe {
                safe_row_count += 1;
            }
        }
    }

    println!(
        "Part 2 - Number of safe rows (including rows made safe by removing one element): {}",
        safe_row_count
    );
}

pub fn run() {
    match read_data_from_txt("src/assets/day2/unusual_data.txt") {
        Ok(data) => {
            part1(&data);
            part2(&data);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
