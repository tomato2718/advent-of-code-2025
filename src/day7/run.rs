use super::laboratories::Laboratories;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();

    println!("Day7:");
    let i = Instant::now();
    println!(
        "  Times will laser split: {}, Time spent: {}µs",
        Laboratories::times_of_splits(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Possible timelines: {}, Time spent: {}µs",
        Laboratories::possible_timelines(&input),
        i.elapsed().as_micros()
    );
}
