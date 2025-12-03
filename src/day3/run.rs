use super::lobby::Lobby;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day3:");
    println!(
        "  Largest joltage with 2 batteries: {}",
        Lobby::find_largest_joltage(&input, 2)
    );
    println!(
        "  Largest joltage with 12 batteries: {}",
        Lobby::find_largest_joltage(&input, 12)
    );
}
