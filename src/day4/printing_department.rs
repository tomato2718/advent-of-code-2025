pub struct PrintingDepartment {}

impl PrintingDepartment {
    pub fn count_accessible_points(department: &Vec<&str>) -> usize {
        let department: Vec<Vec<char>> =
            department.iter().map(|row| row.chars().collect()).collect();

        (0..department.len())
            .flat_map(|r| (0..department[0].len()).map(move |c| (r, c)))
            .filter(|(r, c)| department[*r][*c] == '@')
            .map(|(r, c)| {
                (r.saturating_sub(1)..=(r + 1).min(department.len() - 1))
                    .flat_map(|r| {
                        (c.saturating_sub(1)..=(c + 1).min(department[0].len() - 1))
                            .map(move |c| (r, c))
                    })
                    .filter(|(r, c)| department[*r][*c] == '@')
                    .count()
                    - 1
            })
            .filter(|adjacent_rolls| adjacent_rolls < &4)
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_accessible_points() {
        let department = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        let points = PrintingDepartment::count_accessible_points(&department);

        assert_eq!(points, 13);
    }
}
