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

    pub fn possible_timelines(manifolds: &Vec<&str>) -> usize {
        manifolds
            .iter()
            .fold(
                manifolds[0]
                    .chars()
                    .map(|char| if char == 'S' { 1 } else { 0 })
                    .collect::<Vec<usize>>(),
                |prev, row| {
                    let mut cur = prev.clone();
                    row.chars()
                        .enumerate()
                        .filter(|(index, char)| prev[*index] > 0 && char == &'^')
                        .for_each(|(index, _)| {
                            cur[index - 1] += prev[index];
                            cur[index] = 0;
                            cur[index + 1] += prev[index];
                        });
                    cur
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

    #[test]
    fn possible_timelines() {
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

        let times = Laboratories::possible_timelines(&manifolds);

        assert_eq!(times, 40);
    }
}
