type Lights = Vec<bool>;
type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

pub struct Factory {}

impl Factory {
    pub fn fewest_press(manual: &Vec<(Lights, Buttons, Joltages)>) -> usize {
        0
    }
}
