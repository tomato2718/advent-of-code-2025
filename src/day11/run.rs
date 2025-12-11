use super::reactor::Reactor;
use std::collections::HashMap;

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
    println!("  Different paths: {}", Reactor::different_path(&input));
    println!(
        "  Different paths from svr: {}",
        Reactor::different_path_from_svr(&input)
    );
}
