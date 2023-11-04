use std::{fs, io};

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Movement {
    fn from_str(dir_str: &str) -> Self {
        let mut word_iter = dir_str.split_whitespace();
        let direction_str = word_iter.next().unwrap_or("forward").to_ascii_lowercase();
        let units_str = word_iter.next().unwrap_or("0");

        let units: i32 = units_str.parse().unwrap_or(0);
        return match direction_str.as_str() {
            "forward" => Self::Forward(units),
            "down" => Self::Down(units),
            "up" => Self::Up(units),
            _ => Self::Forward(0),
        };
    }
}

struct SubPosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Default for SubPosition {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl SubPosition {
    fn positon_vector_len(&self) -> i32 {
        self.depth * self.horizontal
    }

    fn apply_movement(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(units) => {
                self.horizontal += units;
            }
            Movement::Down(units) => {
                self.depth += units;
            }
            Movement::Up(units) => {
                self.depth -= units;
            }
        }
    }

    fn apply_movement_v2(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(units) => {
                self.horizontal += units;
                self.depth += units * self.aim;
            }
            Movement::Down(units) => {
                self.aim += units;
            }
            Movement::Up(units) => {
                self.aim -= units;
            }
        }
    }
}

fn load_test_data(path: &str) -> Result<Vec<Movement>, io::Error> {
    let contents = fs::read_to_string(path)?;
    let mut movements = vec![];
    for line in contents.lines() {
        movements.push(Movement::from_str(line));
    }

    Ok(movements)
}

const TEST_DATA_PATH: &'static str = "input.txt";

fn main() -> Result<(), io::Error> {
    let mut sub_position = SubPosition::default();
    let movements = load_test_data(TEST_DATA_PATH)?;

    for movement in movements {
        sub_position.apply_movement_v2(&movement);
    }
    let position_vec_len = sub_position.positon_vector_len();

    println!("Position vector length: {position_vec_len}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direction_from_str() {
        let test_data = [
            ("forward 5", Movement::Forward(5)),
            ("down 4", Movement::Down(4)),
            ("up 10", Movement::Up(10)),
        ];

        for data in test_data {
            assert_eq!(Movement::from_str(data.0), data.1);
        }
    }

    #[test]
    fn test_apply_movement() {
        let moves = [
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];
        let expected_horizontal = 15;
        let expected_depth = 10;

        let mut sub_position = SubPosition::default();
        for movement in moves {
            sub_position.apply_movement(&movement);
        }
        assert_eq!(sub_position.horizontal, expected_horizontal);
        assert_eq!(sub_position.depth, expected_depth);
    }

    #[test]
    fn test_apply_movement_v2() {
        let moves = [
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ];
        let expected_horizontal = 15;
        let expected_depth = 60;

        let mut sub_position = SubPosition::default();
        for movement in moves {
            sub_position.apply_movement_v2(&movement);
        }
        assert_eq!(sub_position.horizontal, expected_horizontal);
        assert_eq!(sub_position.depth, expected_depth);
    }
}
