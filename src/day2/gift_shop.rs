pub struct GiftShop {}

impl GiftShop {
    pub fn add_up_invalid_ids(ids: &Vec<&str>) -> usize {
        GiftShop::parse_ids(ids).fold(0, |acc, (start, end)| {
            acc + (start..=end).filter(GiftShop::is_valid_id).sum::<usize>()
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

    fn is_valid_id(id: &usize) -> bool {
        let s = id.to_string();
        s[0..s.len() / 2] == s[s.len() / 2..]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_up_invalid_ids() {
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

        let res = GiftShop::add_up_invalid_ids(&ids);

        assert_eq!(res, 1227775554);
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
    fn is_valid_id() {
        assert!(GiftShop::is_valid_id(&123123));
        assert!(!GiftShop::is_valid_id(&12321));
    }
}
