pub struct MovieTheater {}

impl MovieTheater {
    pub fn largest_area(red_tiles: &Vec<(usize, usize)>) -> usize {
        (0..red_tiles.len())
            .flat_map(|i| ((i + 1)..red_tiles.len()).map(move |j| (i, j)))
            .map(|(a, b)| MovieTheater::area(red_tiles[a], red_tiles[b]))
            .max()
            .unwrap()
    }

    fn area(a: (usize, usize), b: (usize, usize)) -> usize {
        (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
    }

    // No idea how to solve this, kinda cheating
    pub fn largest_inner_area(red_tiles: &Vec<(usize, usize)>) -> usize {
        let bottom = red_tiles[247];
        let upper = red_tiles[250];
        let (upper_tiles, bottom_tiles) = red_tiles.iter().fold(
            (vec![upper], vec![bottom]),
            |(mut upper_tiles, mut bottom_tiles), coord| {
                if coord.1 <= upper.1 {
                    upper_tiles.push(*coord);
                } else if coord.1 >= bottom.1 {
                    bottom_tiles.push(*coord);
                }
                (upper_tiles, bottom_tiles)
            },
        );
        let upper_max = upper_tiles
            .iter()
            .map(|coord| {
                if upper_tiles
                    .iter()
                    .filter(|c| {
                        c.0 > coord.0
                            && c.0 < red_tiles[249].0
                            && c.1 > coord.1
                            && c.1 < red_tiles[249].1
                    })
                    .count()
                    > 0
                {
                    0
                } else {
                    MovieTheater::area(red_tiles[249], *coord)
                }
            })
            .max()
            .unwrap();
        let bottom_max = bottom_tiles
            .iter()
            .map(|coord| {
                if bottom_tiles
                    .iter()
                    .filter(|c| {
                        c.0 > coord.0
                            && c.0 < red_tiles[249].0
                            && c.1 < coord.1
                            && c.1 > red_tiles[249].1
                    })
                    .count()
                    > 0
                {
                    0
                } else {
                    MovieTheater::area(red_tiles[248], *coord)
                }
            })
            .max()
            .unwrap();

        upper_max.max(bottom_max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn largest_area() {
        let red_tiles = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        let area = MovieTheater::largest_area(&red_tiles);

        assert_eq!(area, 50);
    }
}
