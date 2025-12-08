pub struct TrashCompactor;

impl TrashCompactor {
    pub fn solve_math_work_sheet(numbers: &Vec<&str>, signs: &Vec<&str>) -> usize {
        let mut fixed = vec![Vec::new(); signs.len()];
        numbers
            .iter()
            .map(|row| {
                row.split_whitespace().map(|n| {
                    n.parse::<usize>()
                        .expect("Numbers should contains only digits")
                })
            })
            .for_each(|number| {
                number.enumerate().for_each(|(i, n)| fixed[i].push(n));
            });

        TrashCompactor::solve(&fixed, signs)
    }

    pub fn solve_math_work_sheet_in_correct_format(
        numbers: &Vec<&str>,
        signs: &Vec<&str>,
    ) -> usize {
        let mut fixed = Vec::new();
        let mut cur = vec![];
        for i in (0..numbers[0].len()).rev() {
            let n = numbers.iter().fold(0, |acc, number| {
                if let Ok(n) = number.get(i..=i).unwrap().parse::<usize>() {
                    acc * 10 + n
                } else {
                    acc
                }
            });
            if n == 0 {
                fixed.push(cur);
                cur = vec![];
            } else {
                cur.push(n);
            }
        }
        fixed.push(cur);
        fixed.reverse();

        TrashCompactor::solve(&fixed, signs)
    }

    fn solve(numbers: &Vec<Vec<usize>>, signs: &Vec<&str>) -> usize {
        numbers
            .iter()
            .zip(signs)
            .map(|(number, sign)| {
                if sign == &"*" {
                    number.iter().fold(1, |acc, cur| acc * cur)
                } else {
                    number.iter().fold(0, |acc, cur| acc + cur)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_math_work_sheet() {
        let numbers: Vec<&str> = vec!["123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314"];
        let signs: Vec<&str> = vec!["*", "+", "*", "+"];

        let answer = TrashCompactor::solve_math_work_sheet(&numbers, &signs);

        assert_eq!(answer, 4277556);
    }

    #[test]
    fn solve_math_work_sheet_in_correct_format() {
        let numbers: Vec<&str> = vec!["123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314"];
        let signs: Vec<&str> = vec!["*", "+", "*", "+"];

        let answer = TrashCompactor::solve_math_work_sheet_in_correct_format(&numbers, &signs);

        assert_eq!(answer, 3263827);
    }
}
