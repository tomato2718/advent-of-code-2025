use super::gift_shop::GiftShop;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split(",").collect();
    println!("Day2:");
    let i = Instant::now();
    println!(
        "  Sum of invalid ids: {}, Time spent: {}µs",
        GiftShop::sum_of_id_repeated_twice(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Sum of invalid ids repeated more than twice: {}, Time spent: {}µs",
        GiftShop::sum_of_id_repeated_more_than_twice(&input),
        i.elapsed().as_micros()
    );
}
