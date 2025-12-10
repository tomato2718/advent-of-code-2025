use combination::combine;

type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

pub struct Factory {}

impl Factory {
    pub fn fewest_press(manual: &Vec<(Lights, Buttons, Joltages)>) -> usize {
        manual.iter().map(Factory::find_press).sum()
    }

    fn find_press((lights, buttons, _): &(Lights, Buttons, Joltages)) -> usize {
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
        // TODO: Will need some math to solve this
        0
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
    #[ignore = "not yet implement"]
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
