use super::secret_entrance::SecretEntrance;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day1:");
    println!("  Password: {}", SecretEntrance::find_password(&input));
    println!(
        "  Password with CLICK: {}",
        SecretEntrance::find_password_with_click(&input)
    );
}
