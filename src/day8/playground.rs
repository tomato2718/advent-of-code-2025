use std::collections::{BinaryHeap, HashSet};

pub struct PlayGround {}

impl PlayGround {
    pub fn largest_circuits(junction_boxes: &Vec<(usize, usize, usize)>, pairs: usize) -> usize {
        let mut distances = PlayGround::create_distance_heap(junction_boxes);

        let mut groups: Vec<HashSet<usize>> = Vec::new();

        for _ in 0..pairs {
            let (_, a, b) = distances.pop().unwrap();
            let (mut lhs, mut rhs): (Option<usize>, Option<usize>) = (None, None);
            groups.iter().enumerate().for_each(|(index, group)| {
                if group.contains(&a) {
                    lhs = Some(index);
                }
                if group.contains(&b) {
                    rhs = Some(index);
                }
            });
            if let (Some(l), Some(r)) = (lhs, rhs) {
                if l == r {
                    continue;
                }
                let tmp = groups.get_mut(r).unwrap().to_owned();
                groups.get_mut(l).unwrap().extend(tmp.iter());
                groups.remove(r);
            } else if let Some(i) = lhs {
                groups.get_mut(i).unwrap().insert(b);
            } else if let Some(i) = rhs {
                groups.get_mut(i).unwrap().insert(a);
            } else {
                groups.push(HashSet::from_iter([a, b]));
            };
        }
        groups.sort_by(|a, b| b.len().cmp(&a.len()));
        groups
            .iter()
            .take(3)
            .map(|g| g.len())
            .fold(1, |acc, l| acc * l)
    }

    pub fn last_junction_boxes(junction_boxes: &Vec<(usize, usize, usize)>) -> usize {
        let mut distances = PlayGround::create_distance_heap(junction_boxes);

        let mut groups: Vec<HashSet<usize>> = Vec::new();

        loop {
            let (_, a, b) = distances.pop().unwrap();
            let (mut lhs, mut rhs): (Option<usize>, Option<usize>) = (None, None);
            groups.iter().enumerate().for_each(|(index, group)| {
                if group.contains(&a) {
                    lhs = Some(index);
                }
                if group.contains(&b) {
                    rhs = Some(index);
                }
            });
            if let (Some(l), Some(r)) = (lhs, rhs) {
                if l == r {
                    continue;
                }
                let tmp = groups.get_mut(r).unwrap().to_owned();
                groups.get_mut(l).unwrap().extend(tmp.iter());
                groups.remove(r);
            } else if let Some(i) = lhs {
                groups.get_mut(i).unwrap().insert(b);
            } else if let Some(i) = rhs {
                groups.get_mut(i).unwrap().insert(a);
            } else {
                groups.push(HashSet::from_iter([a, b]));
            };
            if groups.len() == 1
                && groups.iter().map(|g| g.len()).sum::<usize>() == junction_boxes.len()
            {
                return junction_boxes[a].0 * junction_boxes[b].0;
            }
        }
    }

    fn create_distance_heap(
        junction_boxes: &Vec<(usize, usize, usize)>,
    ) -> BinaryHeap<(i64, usize, usize)> {
        (0..junction_boxes.len())
            .flat_map(|a| (0..junction_boxes.len()).map(move |b| (a, b)))
            .filter(|(a, b)| a < b)
            .map(|(a, b)| {
                (
                    -1 * (junction_boxes[a].0.abs_diff(junction_boxes[b].0).pow(2)
                        + junction_boxes[a].1.abs_diff(junction_boxes[b].1).pow(2)
                        + junction_boxes[a].2.abs_diff(junction_boxes[b].2).pow(2))
                        as i64,
                    a,
                    b,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn largest_circuits() {
        let junction_boxes = vec![
            (162, 817, 812),
            (57, 618, 57),
            (906, 360, 560),
            (592, 479, 940),
            (352, 342, 300),
            (466, 668, 158),
            (542, 29, 236),
            (431, 825, 988),
            (739, 650, 466),
            (52, 470, 668),
            (216, 146, 977),
            (819, 987, 18),
            (117, 168, 530),
            (805, 96, 715),
            (346, 949, 466),
            (970, 615, 88),
            (941, 993, 340),
            (862, 61, 35),
            (984, 92, 344),
            (425, 690, 689),
        ];

        let size = PlayGround::largest_circuits(&junction_boxes, 10);

        assert_eq!(size, 40);
    }

    #[test]
    fn last_junction_boxes() {
        let junction_boxes = vec![
            (162, 817, 812),
            (57, 618, 57),
            (906, 360, 560),
            (592, 479, 940),
            (352, 342, 300),
            (466, 668, 158),
            (542, 29, 236),
            (431, 825, 988),
            (739, 650, 466),
            (52, 470, 668),
            (216, 146, 977),
            (819, 987, 18),
            (117, 168, 530),
            (805, 96, 715),
            (346, 949, 466),
            (970, 615, 88),
            (941, 993, 340),
            (862, 61, 35),
            (984, 92, 344),
            (425, 690, 689),
        ];

        let distance = PlayGround::last_junction_boxes(&junction_boxes);

        assert_eq!(distance, 25272);
    }
}
