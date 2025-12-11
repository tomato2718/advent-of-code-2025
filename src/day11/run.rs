use super::reactor::Reactor;
use std::collections::HashMap;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: HashMap<&str, Vec<&str>> = raw_input
        .lines()
        .map(|line| {
            let line = line.split(": ").collect::<Vec<&str>>();
            let (cur, next) = line.split_at(1);
            (cur[0], next[0].split_whitespace().collect())
        })
        .collect();
    println!("Day11:");
    let i = Instant::now();
    println!(
        "  Different paths: {}, Time spent: {}µs",
        Reactor::different_path(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Different paths from svr: {}, Time spent: {}µs",
        Reactor::different_path_from_svr(&input),
        i.elapsed().as_micros()
    );
}
