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
