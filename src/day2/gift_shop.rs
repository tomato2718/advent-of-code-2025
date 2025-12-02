pub struct GiftShop {}

impl GiftShop {
    pub fn sum_of_id_repeated_twice(ids: &Vec<&str>) -> usize {
        GiftShop::parse_ids(ids).fold(0, |acc, (start, end)| {
            acc + (start..=end)
                .filter(GiftShop::repeated_twice)
                .sum::<usize>()
        })
    }

    pub fn sum_of_id_repeated_more_than_twice(ids: &Vec<&str>) -> usize {
        GiftShop::parse_ids(ids).fold(0, |acc, (start, end)| {
            acc + (start..=end)
                .filter(GiftShop::repeated_more_than_twice)
                .sum::<usize>()
        })
    }

    fn parse_ids(ids: &Vec<&str>) -> impl Iterator<Item = (usize, usize)> {
        ids.iter().map(|id| {
            let mut i = (**id).split("-");
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
    }

    fn repeated_twice(id: &usize) -> bool {
        let s = id.to_string();
        s[0..s.len() / 2] == s[s.len() / 2..]
    }

    fn repeated_more_than_twice(id: &usize) -> bool {
        let digits = (*id as f64).log10().floor() as u32 + 1;
        (1..=digits / 2).any(|digit| {
            let mut id = *id;
            let t = 10_usize.pow(digit);
            let want = id % t;
            if (want as f32).log10().floor() as u32 + 1 != digit {
                return false;
            }
            while id > 0 {
                let last = id % t;
                if last != want {
                    return false;
                }
                id /= t;
            }
            true
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_of_id_repeated_twice() {
        let ids = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];

        let res = GiftShop::sum_of_id_repeated_twice(&ids);

        assert_eq!(res, 1227775554);
    }

    #[test]
    fn sum_of_id_repeated_more_than_twice() {
        let ids = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];

        let res = GiftShop::sum_of_id_repeated_more_than_twice(&ids);

        assert_eq!(res, 4174379265);
    }

    #[test]
    fn parse_ids() {
        let ids = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];

        let res: Vec<(usize, usize)> = GiftShop::parse_ids(&ids).collect();

        assert_eq!(
            res,
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124),
            ]
        )
    }

    #[test]
    fn repeated_twice() {
        assert!(GiftShop::repeated_twice(&123123));
        assert!(!GiftShop::repeated_twice(&12321));
    }

    #[test]
    fn repeated_more_than_twice() {
        assert!(GiftShop::repeated_more_than_twice(&111));
        assert!(GiftShop::repeated_more_than_twice(&1212));
        assert!(GiftShop::repeated_more_than_twice(&123123));
        assert!(!GiftShop::repeated_more_than_twice(&12321));
        assert!(!GiftShop::repeated_more_than_twice(&123321));
        assert!(!GiftShop::repeated_more_than_twice(&10101));
    }
}
