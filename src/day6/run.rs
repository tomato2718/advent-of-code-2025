use super::trash_compactor::TrashCompactor;
use std::time::Instant;

pub fn run(raw_input: String) {
    let input = raw_input.split("\n").collect::<Vec<&str>>();
    let mut input_iter = input.into_iter().rev();

    let signs: Vec<&str> = input_iter.next().unwrap().split_whitespace().collect();
    let numbers: Vec<&str> = input_iter.rev().collect();

    println!("Day6:");
    let i = Instant::now();
    println!(
        "  Answer: {}, Time spent: {}µs",
        TrashCompactor::solve_math_work_sheet(&numbers, &signs),
        i.elapsed().as_micros()
    );
    let i = Instant::now();
    println!(
        "  Answer in correct format: {}, Time spent: {}µs",
        TrashCompactor::solve_math_work_sheet_in_correct_format(&numbers, &signs),
        i.elapsed().as_micros()
    );
}
