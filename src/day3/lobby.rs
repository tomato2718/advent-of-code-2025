pub struct Lobby {}

impl Lobby {
    pub fn find_largest_joltage(banks: &Vec<&str>, batteries: usize) -> usize {
        banks
            .iter()
            .map(|bank| {
                bank.chars()
                    .map(|c| c.to_digit(10).expect("Battery shoud contains only digits") as usize)
                    .rev()
            })
            .map(|mut bank| {
                let mut res: Vec<usize> = bank.by_ref().take(batteries).collect();
                res.reverse();

                for battery in bank {
                    if battery < res[0] {
                        continue;
                    };
                    let mut i = 0;
                    while i < batteries - 1 && res[i] >= res[i + 1] {
                        i += 1;
                    }
                    for j in (1..=i).rev() {
                        res[j] = res[j - 1];
                    }
                    res[0] = battery;
                }

                res.iter().fold(0, |acc, cur| acc * 10 + cur)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_largest_joltage_with_2_batteries() {
        let banks = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(Lobby::find_largest_joltage(&banks, 2), 357);
    }

    #[test]
    fn find_largest_joltage_with_12_batteries() {
        let banks = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];

        assert_eq!(Lobby::find_largest_joltage(&banks, 12), 3121910778619);
    }
}
