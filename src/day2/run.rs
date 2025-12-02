use super::gift_shop::GiftShop;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split(",").collect();
    println!("Day2:");
    println!("  Invalid ids: {}", GiftShop::add_up_invalid_ids(&input));
}
