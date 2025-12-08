use super::trash_compactor::TrashCompactor;

pub fn run(raw_input: String) {
    let input = raw_input.split("\n").collect::<Vec<&str>>();
    let mut input_iter = input.iter().rev();

    let signs: Vec<&str> = input_iter.next().unwrap().split_whitespace().collect();
    let numbers: Vec<Vec<usize>> = input_iter
        .map(|row| {
            row.split_whitespace()
                .map(|n| n.parse().expect("Numbers should contains only digits"))
                .collect()
        })
        .rev()
        .collect();

    println!("Day6:");
    println!(
        "  Answer: {}",
        TrashCompactor::solve_math_work_sheet(&numbers, &signs)
    );
}
