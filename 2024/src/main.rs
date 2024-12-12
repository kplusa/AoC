mod puzzles;

fn main() {
    let day = 1;
    match day {
        1 => puzzles::day1::run(),
        _ => println!("Invalid day!"),
    }
}
