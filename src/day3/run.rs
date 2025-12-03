use super::lobby::Lobby;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day3:");
    println!("  Largest joltage: {}", Lobby::find_largest_joltage(&input));
}
