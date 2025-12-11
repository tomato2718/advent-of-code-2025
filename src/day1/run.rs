use super::secret_entrance::SecretEntrance;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day1:");
    let i = Instant::now();
    println!(
        "  Password: {}, Time spent: {}µs",
        SecretEntrance::find_password(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Password with CLICK: {}, Time spent: {}µs",
        SecretEntrance::find_password_with_click(&input),
        i.elapsed().as_micros()
    );
}
