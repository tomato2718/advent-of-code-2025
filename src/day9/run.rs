use super::movie_theater::MovieTheater;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<(usize, usize)> = raw_input
        .lines()
        .map(|line| {
            let mut tmp = line.split(",").into_iter();
            (
                tmp.next().unwrap().parse::<usize>().unwrap(),
                tmp.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!("Day9:");
    let i = Instant::now();
    println!(
        "  Largest area: {}, Time spent: {}µs",
        MovieTheater::largest_area(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Largest inner area: {}, Time spent: {}µs",
        MovieTheater::largest_inner_area(&input),
        i.elapsed().as_micros()
    );
}
