pub struct Cafeteria {}

impl Cafeteria {
    pub fn count_fresh_ingredients(fresh_ranges: &Vec<&str>, available_ids: &Vec<usize>) -> usize {
        let fresh: Vec<(usize, usize)> = Cafeteria::parse_fresh_ranges(fresh_ranges);
        available_ids
            .iter()
            .filter(|id| {
                fresh
                    .iter()
                    .any(|(left_bound, right_bound)| id >= &left_bound && id <= &right_bound)
            })
            .count()
    }

    pub fn count_all_ingredients(fresh_ranges: &Vec<&str>) -> usize {
        let mut fresh: Vec<(usize, usize)> = Cafeteria::parse_fresh_ranges(fresh_ranges);
        fresh.sort();

        fresh
            .into_iter()
            .fold((0, 0), |(mut acc, mut right_bound), (left, right)| {
                if left > right_bound {
                    acc += right - left + 1;
                } else {
                    acc += right.saturating_sub(right_bound);
                };
                right_bound = right_bound.max(right);
                (acc, right_bound)
            })
            .0
    }

    fn parse_fresh_ranges(fresh_ranges: &Vec<&str>) -> Vec<(usize, usize)> {
        fresh_ranges
            .iter()
            .map(|raw_range| {
                let mut i = raw_range.split("-");
                (
                    i.next()
                        .expect("id should contains 2 elements")
                        .parse()
                        .expect("id should be valid digits"),
                    i.next()
                        .expect("id should contains 2 elements")
                        .parse()
                        .expect("id should be valid digits"),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_fresh_ingredient() {
        let fresh_ranges = vec!["3-5", "10-14", "16-20", "12-18"];
        let available_ids = vec![1, 5, 8, 11, 17, 32];

        let total = Cafeteria::count_fresh_ingredients(&fresh_ranges, &available_ids);

        assert_eq!(total, 3);
    }

    #[test]
    fn count_all_ingredients() {
        let fresh_ranges = vec!["3-5", "10-14", "16-20", "12-18", "14-16"];

        let total = Cafeteria::count_all_ingredients(&fresh_ranges);

        assert_eq!(total, 14);
    }
}
