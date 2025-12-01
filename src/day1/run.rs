use super::secret_entrance::SecretEntrance;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day1:");
    println!("  password: {}", SecretEntrance::find_password(&input));
    println!(
        "  password with CLICK: {}",
        SecretEntrance::find_password_with_click(&input)
    );
}
