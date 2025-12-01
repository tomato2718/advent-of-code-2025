use advent_of_code_2025::day1;
use std::fs::read_to_string;

fn main() {
    [("day1", day1::run)]
        .into_iter()
        .for_each(|(name, runnable)| runnable(read_file(name)));
}

fn read_file(name: &str) -> String {
    match read_to_string(format!(".inputs/{}.txt", name)) {
        Ok(s) => s,
        _ => panic!("Failed to open file"),
    }
}
