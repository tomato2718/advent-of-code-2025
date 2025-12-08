pub struct TrashCompactor;

impl TrashCompactor {
    pub fn solve_math_work_sheet(numbers: &Vec<Vec<usize>>, signs: &Vec<&str>) -> usize {
        numbers
            .iter()
            .zip(std::iter::repeat(signs))
            .fold(
                signs
                    .iter()
                    .map(|sign| if sign == &"+" { 0 } else { 1 })
                    .collect::<Vec<usize>>(),
                |acc, (number, sign)| {
                    acc.into_iter()
                        .zip(number)
                        .zip(sign)
                        .map(|((a, n), s)| if s == &"+" { a + n } else { a * n })
                        .collect()
                },
            )
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_math_work_sheet() {
        let numbers: Vec<Vec<usize>> = vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314],
        ];
        let signs: Vec<&str> = vec!["*", "+", "*", "+"];

        let answer = TrashCompactor::solve_math_work_sheet(&numbers, &signs);

        assert_eq!(answer, 4277556);
    }
}
