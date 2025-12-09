use super::movie_theater::MovieTheater;

pub fn run(raw_input: String) {
    let input: Vec<(usize, usize)> = raw_input
        .lines()
        .map(|line| {
            let mut tmp = line.split(",").into_iter();
            (
                tmp.next().unwrap().parse::<usize>().unwrap(),
                tmp.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    println!("Day9:");
    println!("  Largest area: {}", MovieTheater::largest_area(&input));
}
