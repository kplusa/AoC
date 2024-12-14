use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn read_data_from_txt<P: AsRef<Path>>(filepath: P) -> io::Result<String> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut data = String::new();

    for line in reader.lines() {
        data.push_str(&line?);
    }

    Ok(data)
}

fn extract_multiplications(data: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut multiplications = Vec::new();

    for cap in re.captures_iter(data) {
        let num1 = cap[1].parse::<i32>().unwrap();
        let num2 = cap[2].parse::<i32>().unwrap();
        multiplications.push((num1, num2));
    }

    multiplications
}

pub fn part1(multiplications: &Vec<(i32, i32)>) {
    let mut total_sum: i64 = 0; 

    for &(num1, num2) in multiplications {
        total_sum += (num1 as i64) * (num2 as i64);
    }

    println!("Part 1 - Total Sum of Multiplications: {}", total_sum);
}

pub fn run() {
    match read_data_from_txt("src/assets/day3/mull_it_over.txt") {
        Ok(data) => {
            let multiplications = extract_multiplications(&data);
            part1(&multiplications);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}