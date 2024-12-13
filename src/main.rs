mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The advent of code day to run
    #[arg(short, long)]
    day: u8,

    /// The part of the advent of code day to run
    #[arg(short, long)]
    part: u8,
}

pub fn main() {
    let Args { day, part } = Args::parse();

    match (day, part) {
        (1, 1) => println!("day 1, part 1: {}", day1::part_1(None)),
        (1, 2) => println!("day 1, part 2: {}", day1::part_2(None)),
        (2, 1) => println!("day 2, part 1: {}", day2::part_1(None)),
        (2, 2) => println!("day 2, part 2: {}", day2::part_2(None)),
        (3, 1) => println!("day 3, part 1: {}", day3::part_1(None)),
        (3, 2) => println!("day 3, part 2: {}", day3::part_2(None)),
        (4, 1) => println!("day 4, part 1: {}", day4::part_1(None)),
        (4, 2) => println!("day 4, part 2: {}", day4::part_2(None)),
        (5, 1) => println!("day 5, part 1: {}", day5::part_1(None)),
        (5, 2) => println!("day 5, part 2: {}", day5::part_2(None)),
        (6, 1) => println!("day 6, part 1: {}", day6::part_1(None)),
        (6, 2) => println!("day 6, part 2: {}", day6::part_2(None)),
        (7, 1) => println!("day 7, part 1: {}", day7::part_1(None)),
        (7, 2) => println!("day 7, part 2: {}", day7::part_2(None)),
        (8, 1) => println!("day 8, part 1: {}", day8::part_1(None)),
        (8, 2) => println!("day 8, part 2: {}", day8::part_2(None)),
        (9, 1) => println!("day 9, part 1: {}", day9::part_1(None)),
        (9, 2) => println!("day 9, part 2: {}", day9::part_2(None)),
        (10, 1) => println!("day 10, part 1: {}", day10::part_1(None)),
        (10, 2) => println!("day 10, part 2: {}", day10::part_2(None)),
        (11, 1) => println!("day 11, part 1: {}", day11::part_1(None, 25)),
        (11, 2) => println!("day 11, part 2: {}", day11::part_1(None, 75)),
        (12, 1) => println!("day 12, part 1: {}", day12::part_1(None)),
        (12, 2) => println!("day 12, part 2: {}", day12::part_2(None)),
        (13, 1) => println!("day 13, part 1: {}", day13::part_1(None)),
        (13, 2) => println!("day 13, part 2: {}", day13::part_2(None)),
        _ => panic!("Invalid day or part"),
    }
}
