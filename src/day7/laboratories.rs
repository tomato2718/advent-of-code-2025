pub struct Laboratories {}

impl Laboratories {
    pub fn times_of_splits(manifolds: &Vec<&str>) -> usize {
        manifolds
            .iter()
            .fold(
                (0, manifolds[0].replace("S", "|")),
                |(mut acc, prev), row| {
                    let mut cur = prev.clone();
                    row.chars()
                        .enumerate()
                        .filter(|(index, char)| &prev[*index..=*index] == "|" && char == &'^')
                        .for_each(|(index, _)| {
                            acc += 1;
                            cur.replace_range((index - 1)..=(index + 1), "|.|");
                        });
                    (acc, cur)
                },
            )
            .0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn times_of_splits() {
        let manifolds = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let times = Laboratories::times_of_splits(&manifolds);

        assert_eq!(times, 21);
    }
}
