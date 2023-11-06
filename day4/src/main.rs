use std::fs;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BingoSpace {
    value: u32,
    marked: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BingoBoard {
    spaces: [[BingoSpace; 5]; 5],
    marked_count: u32,
    is_winner: bool,
    winning_val: Option<u32>,
    placement: Option<usize>,
}

impl BingoBoard {
    pub fn new(values: &[Vec<u32>]) -> Self {
        let mut bingo_spaces = [[BingoSpace {
            value: 0,
            marked: false,
        }; 5]; 5];
        for y in 0..5 {
            let row = &values[y];
            for x in 0..5 {
                bingo_spaces[y][x].value = row[x];
            }
        }

        Self {
            spaces: bingo_spaces,
            marked_count: 0,
            is_winner: false,
            winning_val: None,
            placement: None,
        }
    }

    fn column_is_winner(&self, col_idx: usize) -> bool {
        let mut marked_count = 0;
        for y in 0..5 {
            let space = &self.spaces[y][col_idx];
            if space.marked {
                marked_count += 1;
            }
        }

        marked_count == 5
    }

    /// Updates the [BingoBoard]'s state to match whether it is a winner
    pub fn determine_if_winner(&mut self, value: u32, placement: &mut usize) {
        if self.is_winner {
            return;
        }

        if self.marked_count < 5 {
            self.is_winner = false;
            return;
        }

        for y in 0..self.spaces.len() {
            let mut marked_count = 0;
            let row = &self.spaces[y];
            for x in 0..row.len() {
                if y == 0 {
                    // Make sure to check columns only for the first row
                    if self.column_is_winner(x) {
                        self.is_winner = true;
                        self.winning_val = Some(value);
                        self.placement = Some(*placement);
                        *placement += 1;
                        return;
                    }
                }
                let space = &self.spaces[y][x];
                if space.marked {
                    marked_count += 1;
                }
            }
            if marked_count == 5 {
                self.is_winner = true;
                self.winning_val = Some(value);
                self.placement = Some(*placement);
                *placement += 1;
                return;
            }
        }

        self.is_winner = false;
    }

    pub fn sum_of_unmarked(&self) -> u32 {
        self.spaces
            .iter()
            .flatten()
            .filter(|&space| !space.marked)
            .fold(0, |acc, space| acc + space.value)
    }

    pub fn mark_if_present(&mut self, value: u32, placement: &mut usize) {
        self.spaces
            .iter_mut()
            .flatten()
            .filter(|space| space.value == value)
            .for_each(|space| {
                space.marked = true;
                self.marked_count += 1;
            });
        self.determine_if_winner(value, placement);
    }

    pub fn calculate_score(&self) -> u32 {
        self.sum_of_unmarked() * self.winning_val.unwrap_or(0)
    }
}

fn load_input_data(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    // Load values from first line
    let values_line = lines.next().unwrap();
    let values: Vec<u32> = values_line
        .split(",")
        .map(|val_str| val_str.parse().unwrap())
        .collect();

    // Load BingoBoards from remaining lines
    let board_inputs: Vec<Vec<u32>> = lines
        .filter(|&line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut boards = vec![];
    for i in (0..board_inputs.len()).step_by(5) {
        let stop = i + 5;
        let input_rows = &board_inputs[i..stop];
        boards.push(BingoBoard::new(input_rows));
    }

    (values, boards)
}

/// Runs all of the Bingo games; determining winners, the order in which they won, and the associated winning values
fn run_game(vals: &[u32], boards: &mut [BingoBoard]) -> (Option<BingoBoard>, Option<BingoBoard>) {
    let mut placement = 0;
    for i in 0..vals.len() {
        let val = vals[i];
        for board_idx in 0..boards.len() {
            let board = &mut boards[board_idx];
            if !board.is_winner {
                board.mark_if_present(val, &mut placement);
            }
        }
    }

    let first_winner = boards.iter().find(|&board| {
        return match board.placement {
            Some(placement) => placement == 0,
            None => false,
        };
    });

    let first_winner = match first_winner {
        Some(board) => Some(board.clone()),
        None => None,
    };

    let last_winner = boards
        .iter()
        .filter(|&board| board.is_winner && board.placement.is_some())
        .max_by(|&x, &y| x.placement.cmp(&y.placement));

    let last_winner = match last_winner {
        Some(board) => Some(board.clone()),
        None => None,
    };

    (first_winner, last_winner)
}

fn main() -> anyhow::Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let (values, mut bingo_boards) = load_input_data(&input_str);
    drop(input_str);

    let (first_winner, last_winner) = run_game(&values, &mut bingo_boards);

    match first_winner {
        Some(board) => {
            let score = board.calculate_score();
            println!("First winner's score: {score}");
        }
        None => {}
    }

    match last_winner {
        Some(board) => {
            let score = board.calculate_score();
            println!("Last winner's score: {score}");
        }
        None => {}
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    impl BingoSpace {
        fn new(value: u32) -> Self {
            Self {
                value,
                marked: false,
            }
        }
    }

    const TEST_INPUT_DATA: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
        \n\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19\n\
        \n\
         3 15  0  2 22\n\
         9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
        \n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7";

    const TEST_INPUT_LINE_COUNT: usize = 19;

    const TEST_VALUES: [u32; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    lazy_static! {
        static ref BINGO_VAL_ARRAY: Vec<Vec<u32>> = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ];
        static ref BINGO_BOARD: BingoBoard = BingoBoard {
            spaces: [
                [
                    BingoSpace::new(22),
                    BingoSpace::new(13),
                    BingoSpace::new(17),
                    BingoSpace::new(11),
                    BingoSpace::new(0),
                ],
                [
                    BingoSpace::new(8),
                    BingoSpace::new(2),
                    BingoSpace::new(23),
                    BingoSpace::new(4),
                    BingoSpace::new(24),
                ],
                [
                    BingoSpace::new(21),
                    BingoSpace::new(9),
                    BingoSpace::new(14),
                    BingoSpace::new(16),
                    BingoSpace::new(7),
                ],
                [
                    BingoSpace::new(6),
                    BingoSpace::new(10),
                    BingoSpace::new(3),
                    BingoSpace::new(18),
                    BingoSpace::new(5),
                ],
                [
                    BingoSpace::new(1),
                    BingoSpace::new(12),
                    BingoSpace::new(20),
                    BingoSpace::new(15),
                    BingoSpace::new(19),
                ],
            ],
            is_winner: false,
            marked_count: 0,
            winning_val: None,
            placement: None
        };
    }

    #[test]
    fn test_new_bingo_board() {
        assert_eq!(BingoBoard::new(&BINGO_VAL_ARRAY), *BINGO_BOARD);
    }

    #[test]
    fn test_column_is_winner() {
        let mut test_board = BINGO_BOARD.clone();
        let marked_col = 3;
        // Mark all spaces in column 3
        for y in 0..5 {
            test_board.spaces[y][marked_col].marked = true;
        }

        assert_eq!(test_board.column_is_winner(2), false);
        assert_eq!(test_board.column_is_winner(marked_col), true);
    }

    #[test]
    fn test_sum_of_unmarked() {
        let mut test_board = BINGO_BOARD.clone();
        for y in 0..4 {
            for x in 0..5 {
                test_board.spaces[y][x].marked = true;
            }
        }
        let expected_sum = 67;

        assert_eq!(test_board.sum_of_unmarked(), expected_sum);
    }

    #[test]
    fn test_load_input_data() {
        assert_eq!(TEST_INPUT_DATA.lines().count(), TEST_INPUT_LINE_COUNT);

        let (values, bingo_boards) = load_input_data(TEST_INPUT_DATA);
        assert_eq!(values.as_slice(), TEST_VALUES);

        assert_eq!(bingo_boards.len(), 3);
    }

    #[test]
    fn test_run_game() {
        let (values, mut bingo_boards) = load_input_data(TEST_INPUT_DATA);
        let (first_winner, last_winner) = run_game(&values, &mut bingo_boards);

        assert!(first_winner.is_some());
        // assert_eq!(first_winner, Some(expected_first_winner));
        assert!(last_winner.is_some());
        // assert_eq!(last_winner, Some(expected_last_winner));
    }
}
