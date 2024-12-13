mod puzzles;

fn main() {
    let day = 2;
    match day {
        1 => puzzles::day1::run(),
        2 => puzzles::day2::run(),
        _ => println!("Invalid day!"),
    }
}
