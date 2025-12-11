use super::lobby::Lobby;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day3:");
    let i = Instant::now();
    println!(
        "  Largest joltage with 2 batteries: {}, Time spent: {}µs",
        Lobby::find_largest_joltage(&input, 2),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Largest joltage with 12 batteries: {}, Time spent: {}µs",
        Lobby::find_largest_joltage(&input, 12),
        i.elapsed().as_micros()
    );
}
