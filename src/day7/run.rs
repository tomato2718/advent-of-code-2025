use super::laboratories::Laboratories;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();

    println!("Day7:");
    println!(
        "  Times will laser split: {}",
        Laboratories::times_of_splits(&input)
    );
}
