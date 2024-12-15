mod puzzles;

fn main() {
    let day = 4;
    match day {
        1 => puzzles::day1::run(),
        2 => puzzles::day2::run(),
        3 => puzzles::day3::run(),
        4 => puzzles::day4::run(),
        _ => println!("Invalid day!"),
    }
}
