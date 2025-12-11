use super::cafeteria::Cafeteria;
use std::time::Instant;

pub fn run(raw_input: String) {
    let mut input = raw_input.split("\n\n");
    let fresh_ranges = input
        .next()
        .expect("Input should contains 2 parts")
        .lines()
        .collect();
    let available_ids = input
        .next()
        .expect("Input should contains 2 parts")
        .lines()
        .map(|id| id.parse().expect("Id should be digits"))
        .collect();
    println!("Day5:");
    let i = Instant::now();
    println!(
        "  Fresh ingredients counts: {}, Time spent: {}µs",
        Cafeteria::count_fresh_ingredients(&fresh_ranges, &available_ids),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  All ingredients counts: {}, Time spent: {}µs",
        Cafeteria::count_all_ingredients(&fresh_ranges),
        i.elapsed().as_micros()
    );
}
