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

fn extract_multiplications(data: &str) -> Vec<(i32, i32, usize)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut multiplications = Vec::new();

    for mat in re.find_iter(data) {
        let cap = re.captures(mat.as_str()).unwrap();
        let num1 = cap[1].parse::<i32>().unwrap();
        let num2 = cap[2].parse::<i32>().unwrap();
        multiplications.push((num1, num2, mat.start()));
    }

    multiplications
}

pub fn part1(multiplications: &Vec<(i32, i32, usize)>) {
    let mut total_sum: i64 = 0;

    for &(num1, num2, _) in multiplications {
        total_sum += (num1 as i64) * (num2 as i64);
    }

    println!("Part 1 - Total Sum of Multiplications: {}", total_sum);
}

pub fn part2(data: &str, multiplications: &Vec<(i32, i32, usize)>) {
    let mut total_sum: i64 = 0;
    let mut is_mul_enabled = true;

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut do_dont_positions: Vec<(usize, bool)> = Vec::new();
    for mat in do_re.find_iter(data) {
        do_dont_positions.push((mat.start(), true));
    }
    for mat in dont_re.find_iter(data) {
        do_dont_positions.push((mat.start(), false));
    }
    do_dont_positions.sort();

    for &(num1, num2, mul_pos) in multiplications {
        is_mul_enabled = true;
        for &(pos, is_do) in do_dont_positions.iter().rev() {
            if pos < mul_pos {
                is_mul_enabled = is_do;
                break;
            }
        }

        if is_mul_enabled {
            total_sum += (num1 as i64) * (num2 as i64);
        }
    }

    println!("Part 2 - Total Sum (with do/don't): {}", total_sum);
}

pub fn run() {
    match read_data_from_txt("src/assets/day3/mull_it_over.txt") {
        Ok(data) => {
            let multiplications = extract_multiplications(&data);
            part1(&multiplications);
            println!("");
            part2(&data, &multiplications);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}