use combination::combine;
use std::collections::HashMap;

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

pub struct Factory {}

impl Factory {
    pub fn fewest_press(manual: &Vec<(Lights, Buttons, Joltages)>) -> usize {
        manual
            .iter()
            .map(|(lights, buttons, _)| Factory::find_press(lights, buttons))
            .sum()
    }

    fn find_press(lights: &Lights, buttons: &Buttons) -> usize {
        for comb in combine::from_vec(buttons) {
            if Factory::is_valid(lights, &comb) {
                return comb.len();
            }
        }
        panic!("Impossible buttons to turn all lights on");
    }

    fn is_valid(lights: &Lights, buttons: &Buttons) -> bool {
        let mut pressed = vec![false; lights.len()];
        for button in buttons {
            for light in button {
                pressed[*light] = !pressed[*light];
            }
        }
        &pressed == lights
    }

    pub fn fewest_press_met_joltage_requirement(
        manual: &Vec<(Lights, Buttons, Joltages)>,
    ) -> usize {
        manual
            .iter()
            .map(|(_, buttons, joltages)| {
                Factory::solve(
                    buttons,
                    joltages,
                    &Factory::create_patterns(buttons, joltages),
                    &mut HashMap::new(),
                )
            })
            .sum()
    }

    fn create_patterns(buttons: &Buttons, joltages: &Joltages) -> HashMap<Vec<usize>, usize> {
        let mut patterns = HashMap::new();
        patterns.insert(vec![0; joltages.len()], 0);
        for comb in combine::from_vec(buttons) {
            let lights = comb.iter().fold(vec![0; joltages.len()], |mut lights, c| {
                c.iter().for_each(|d| lights[*d] += 1);
                lights
            });
            if !patterns.contains_key(&lights) {
                patterns.insert(lights, comb.len());
            }
        }
        patterns
    }

    fn solve(
        buttons: &Buttons,
        joltages: &Joltages,
        patterns: &HashMap<Vec<usize>, usize>,
        cache: &mut HashMap<Joltages, usize>,
    ) -> usize {
        if joltages.iter().all(|j| j == &0) {
            return 0;
        }
        patterns
            .iter()
            .flat_map(|(pattern, press)| {
                let mut remain: Vec<usize> = joltages
                    .iter()
                    .zip(pattern)
                    .map(|(j, p)| if j >= p { j - p } else { 1 })
                    .collect();
                if remain.iter().any(|r| r & 1 == 1) {
                    return None;
                }
                remain = remain.iter().map(|r| r / 2).collect();
                if !cache.contains_key(&remain) {
                    let count = Factory::solve(buttons, &remain, patterns, cache);
                    cache.insert(remain.clone(), count);
                }
                return Some(press + (2 * cache[&remain]));
            })
            .min()
            .unwrap_or(10_000_000)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fewest_press() {
        let inputs = vec![
            (
                vec![false, true, true, false],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                vec![3, 5, 4, 7],
            ),
            (
                vec![false, false, false, true, false],
                vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                vec![7, 5, 12, 7, 2],
            ),
            (
                vec![false, true, true, true, false, true],
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                vec![10, 11, 11, 5, 10, 5],
            ),
        ];

        let count = Factory::fewest_press(&inputs);

        assert_eq!(count, 7);
    }

    #[test]
    fn fewest_press_met_joltage_requirement() {
        let inputs = vec![
            (
                vec![false, true, true, false],
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                vec![3, 5, 4, 7],
            ),
            (
                vec![false, false, false, true, false],
                vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                vec![7, 5, 12, 7, 2],
            ),
            (
                vec![false, true, true, true, false, true],
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                vec![10, 11, 11, 5, 10, 5],
            ),
        ];

        let count = Factory::fewest_press_met_joltage_requirement(&inputs);

        assert_eq!(count, 33);
    }
}
