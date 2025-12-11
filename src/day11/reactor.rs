use std::collections::HashMap;

pub struct Reactor {}

impl Reactor {
    pub fn different_path(devices: &HashMap<&str, Vec<&str>>) -> usize {
        Reactor::visit("you", "out", devices, &mut HashMap::new())
    }

    pub fn different_path_from_svr(devices: &HashMap<&str, Vec<&str>>) -> usize {
        Reactor::visit("svr", "fft", devices, &mut HashMap::new())
            * Reactor::visit("fft", "dac", devices, &mut HashMap::new())
            * Reactor::visit("dac", "out", devices, &mut HashMap::new())
    }

    fn visit<'a>(
        cur: &str,
        want: &str,
        devices: &HashMap<&str, Vec<&'a str>>,
        visited: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if cur == want {
            return 1;
        }
        if let Some(n) = devices.get(cur) {
            n.iter()
                .map(|next| {
                    if !visited.contains_key(next) {
                        let count = Reactor::visit(next, want, devices, visited);
                        visited.insert(next, count);
                    }
                    visited[next]
                })
                .sum()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dirrerent_path() {
        let input: HashMap<&str, Vec<&str>> = HashMap::from_iter([
            ("aaa", vec!["you", "hhh"]),
            ("you", vec!["bbb", "ccc"]),
            ("bbb", vec!["ddd", "eee"]),
            ("ccc", vec!["ddd", "eee", "fff"]),
            ("ddd", vec!["ggg"]),
            ("eee", vec!["out"]),
            ("fff", vec!["out"]),
            ("ggg", vec!["out"]),
            ("hhh", vec!["ccc", "fff", "iii"]),
            ("iii", vec!["out"]),
        ]);

        let paths = Reactor::different_path(&input);

        assert_eq!(paths, 5);
    }

    #[test]
    fn dirrerent_svr_path() {
        let input: HashMap<&str, Vec<&str>> = HashMap::from_iter([
            ("svr", vec!["aaa", "bbb"]),
            ("aaa", vec!["fft"]),
            ("fft", vec!["ccc"]),
            ("bbb", vec!["tty"]),
            ("tty", vec!["ccc"]),
            ("ccc", vec!["ddd", "eee"]),
            ("ddd", vec!["hub"]),
            ("hub", vec!["fff"]),
            ("eee", vec!["dac"]),
            ("dac", vec!["fff"]),
            ("fff", vec!["ggg", "hhh"]),
            ("ggg", vec!["out"]),
            ("hhh", vec!["out"]),
        ]);

        let paths = Reactor::different_path_from_svr(&input);

        assert_eq!(paths, 2);
    }
}
