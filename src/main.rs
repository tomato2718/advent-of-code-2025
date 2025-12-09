use advent_of_code_2025::{day1, day2, day3, day4, day5, day6, day7, day8, day9};
use std::fs::read_to_string;

fn main() {
    ([
        ("day1", day1::run as fn(String)),
        ("day2", day2::run),
        ("day3", day3::run),
        ("day4", day4::run),
        ("day5", day5::run),
        ("day6", day6::run),
        ("day7", day7::run),
        ("day8", day8::run),
        ("day9", day9::run),
    ])
    .into_iter()
    .for_each(|(name, runnable)| runnable(read_file(name)));
}

fn read_file(name: &str) -> String {
    match read_to_string(format!(".inputs/{}.txt", name)) {
        Ok(s) => s,
        _ => panic!("Failed to open file"),
    }
}
