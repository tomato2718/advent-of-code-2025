const START: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub struct SecretEntrance {}

impl SecretEntrance {
    pub fn find_password(rotations: &Vec<&str>) -> usize {
        let mut acc = 0;
        let mut cur = START;
        for command in rotations.iter() {
            cur = (cur + SecretEntrance::parse_command(command)) % DIAL_SIZE;
            if cur == 0 {
                acc += 1;
            }
        }

        acc
    }

    /// 0x434C49434B => CLICK
    pub fn find_password_with_click(rotations: &Vec<&str>) -> usize {
        let mut acc = 0;
        let mut cur = START;
        let mut prev = cur;
        for command in rotations.iter() {
            cur += SecretEntrance::parse_command(command);
            acc += (cur / DIAL_SIZE).abs() as usize;
            if cur <= 0 && prev != 0 {
                acc += 1;
            };
            cur = ((cur % DIAL_SIZE) + DIAL_SIZE) % DIAL_SIZE;
            prev = cur;
        }

        acc as usize
    }

    fn parse_command(command: &str) -> i32 {
        let sign = match &command[0..=0] {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid command"),
        };
        let turns = match command[1..].parse::<i32>() {
            Ok(turns) => turns,
            Err(_) => panic!("Invalid command"),
        };
        sign * turns
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_password() {
        let rotations = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let res = SecretEntrance::find_password(&rotations);

        assert_eq!(res, 3);
    }

    #[test]
    fn find_password_with_click() {
        let rotations = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let res = SecretEntrance::find_password_with_click(&rotations);

        assert_eq!(res, 6);
    }
}
