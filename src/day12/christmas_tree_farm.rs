type Region = (usize, usize);
type Present = Vec<usize>;

pub struct ChristmasTreeFarm {}

impl ChristmasTreeFarm {
    pub fn regions_can_fit_presents(regions: &Vec<(Region, Present)>) -> usize {
        regions
            .iter()
            .filter(|(region, present)| {
                region.0 * region.1 >= present.iter().sum::<usize>() * 9 - 2
            })
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regions_can_fit_presents() {
        let regions = vec![
            ((4, 4), vec![0, 0, 0, 0, 2, 0]),
            ((12, 5), vec![1, 0, 1, 0, 2, 2]),
            ((12, 5), vec![1, 0, 1, 0, 3, 2]),
        ];

        let count = ChristmasTreeFarm::regions_can_fit_presents(&regions);

        assert_eq!(count, 2);
    }
}
