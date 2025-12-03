pub struct Lobby {}

impl Lobby {
    pub fn find_largest_joltage(banks: &Vec<&str>) -> usize {
        banks.iter().map(|bank| Lobby::largest_joltage(*bank)).sum()
    }

    fn largest_joltage(bank: &str) -> usize {
        bank.chars()
            .rev()
            .fold([None, None], |[max, second], battery| {
                let b = battery
                    .to_digit(10)
                    .expect("Battery shoud contains only digits") as usize;
                if second.is_none() {
                    [None, Some(b)]
                } else if max.is_none() {
                    [Some(b), second]
                } else if b >= max.unwrap() {
                    [Some(b), max.max(second)]
                } else {
                    [max, second]
                }
            })
            .iter()
            .flatten()
            .fold(0, |acc, cur| acc * 10 + cur)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_largest_joltage() {
        let banks = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(Lobby::find_largest_joltage(&banks), 357);
    }
}
