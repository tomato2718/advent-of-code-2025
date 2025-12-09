use super::playground::PlayGround;

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

    println!("Day6:");
    println!(
        "  Pruduct of three largest circuits: {}",
        PlayGround::largest_circuits(&input, 1000)
    );
}
