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

    pub fn count_accessible_points_with_removal(department: &Vec<&str>) -> usize {
        let (row_count, col_count) = (department.len(), department[0].len());
        let mut department: Vec<Vec<char>> =
            department.iter().map(|row| row.chars().collect()).collect();

        let mut total = 0;
        loop {
            let removed = (0..row_count)
                .flat_map(|r| (0..col_count).map(move |c| (r, c)))
                .filter(|(r, c)| department[*r][*c] == '@')
                .collect::<Vec<(usize, usize)>>()
                .into_iter()
                .fold(0, |acc, (r, c)| {
                    let adjacent_rolls = (r.saturating_sub(1)..=(r + 1).min(row_count - 1))
                        .flat_map(|r| {
                            (c.saturating_sub(1)..=(c + 1).min(col_count - 1)).map(move |c| (r, c))
                        })
                        .filter(|(r, c)| department[*r][*c] == '@')
                        .count()
                        - 1;
                    if adjacent_rolls < 4 {
                        department[r][c] = '.';
                        acc + 1
                    } else {
                        acc
                    }
                });
            if removed == 0 {
                break;
            }
            total += removed;
        }
        total
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

    #[test]
    fn count_accessible_points_with_removal() {
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

        let points = PrintingDepartment::count_accessible_points_with_removal(&department);

        assert_eq!(points, 43);
    }
}
