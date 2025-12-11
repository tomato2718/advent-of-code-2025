use super::printing_department::PrintingDepartment;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day4:");
    let i = Instant::now();
    println!(
        "  Accessible points: {}, Time spent: {}µs",
        PrintingDepartment::count_accessible_points(&input),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Accessible points with removal: {}, Time spent: {}µs",
        PrintingDepartment::count_accessible_points_with_removal(&input),
        i.elapsed().as_micros()
    );
}
