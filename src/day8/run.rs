use super::playground::PlayGround;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input = raw_input
        .lines()
        .map(|line| {
            let mut split = line.split(",");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    println!("Day8:");
    let i = Instant::now();
    println!(
        "  Pruduct of three largest circuits: {}, Time spent: {}µs",
        PlayGround::largest_circuits(&input, 1000),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Pruduct of x-coord of last 2 junction boxes: {}, Time spent: {}µs",
        PlayGround::last_junction_boxes(&input),
        i.elapsed().as_micros()
    );
}
