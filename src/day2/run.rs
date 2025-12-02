use super::gift_shop::GiftShop;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split(",").collect();
    println!("Day2:");
    println!(
        "  Sum of invalid ids: {}",
        GiftShop::sum_of_id_repeated_twice(&input)
    );
    println!(
        "  Sum of invalid ids repeated more than twice: {}",
        GiftShop::sum_of_id_repeated_more_than_twice(&input)
    );
}
