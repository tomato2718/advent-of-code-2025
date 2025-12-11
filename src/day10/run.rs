use super::factory::Factory;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> = raw_input
        .lines()
        .map(|line| {
            let mut tmp: Vec<&str> = line.split(" ").collect();
            let mut button = tmp.split_off(1);
            let joltage = button.split_off(button.len() - 1);
            (tmp[0], button, joltage[0])
        })
        .map(|(light, button, joltage)| {
            (
                light
                    .chars()
                    .flat_map(|char| {
                        if char == '.' {
                            Some(false)
                        } else if char == '#' {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .collect(),
                button
                    .iter()
                    .map(|b| {
                        b[1..b.len() - 1]
                            .split(",")
                            .map(|j| j.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect(),
                joltage[1..joltage.len() - 1]
                    .split(",")
                    .map(|j| j.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect();
    println!("Day10:");
    let i = Instant::now();
    println!(
        "  Fewest press: {}, Time spent: {}µs",
        Factory::fewest_press(&input),
        i.elapsed().as_micros()
    );
}
