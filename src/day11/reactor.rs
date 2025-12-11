use std::collections::HashMap;

pub struct Reactor {}

impl Reactor {
    pub fn different_path(devices: &HashMap<&str, Vec<&str>>) -> usize {
        Reactor::visit("you", devices, &mut HashMap::new())
    }

    fn visit<'a>(
        cur: &str,
        devices: &HashMap<&str, Vec<&'a str>>,
        visited: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if cur == "out" {
            return 1;
        }
        devices[cur]
            .iter()
            .map(|next| {
                if !visited.contains_key(next) {
                    let count = Reactor::visit(next, devices, visited);
                    visited.insert(next, count);
                }
                visited[next]
            })
            .sum()
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
}
