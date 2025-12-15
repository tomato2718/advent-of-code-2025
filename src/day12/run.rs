use super::christmas_tree_farm::ChristmasTreeFarm;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input = raw_input
        .split("\n\n")
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.split(": ").collect::<Vec<&str>>();
            let (region, presents) = line.split_at(1);
            let mut region = region[0].split("x");
            (
                (
                    region.next().unwrap().parse::<usize>().unwrap(),
                    region.next().unwrap().parse::<usize>().unwrap(),
                ),
                presents[0]
                    .split_whitespace()
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();
    println!("Day12:");
    let i = Instant::now();
    println!(
        "  Regions can fit presents: {}, Time spent: {}µs",
        ChristmasTreeFarm::regions_can_fit_presents(&input),
        i.elapsed().as_micros()
    );
}
