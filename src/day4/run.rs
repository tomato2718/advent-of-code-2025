use super::printing_department::PrintingDepartment;

pub fn run(raw_input: String) {
    let input: Vec<&str> = raw_input.split("\n").collect();
    println!("Day4:");
    println!(
        "  Accessible points: {}",
        PrintingDepartment::count_accessible_points(&input)
    );
    println!(
        "  Accessible points with removal: {}",
        PrintingDepartment::count_accessible_points_with_removal(&input)
    );
}
