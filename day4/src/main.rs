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

    pub fn is_winner(&self) -> bool {
        for y in 0..self.spaces.len() {
            let mut marked_count = 0;
            let row = &self.spaces[y];
            for x in 0..row.len() {
                if y == 0 {
                    // Make sure to check columns only for the first row
                    if self.column_is_winner(x) {
                        return true;
                    }
                }
                let space = &self.spaces[y][x];
                if space.marked {
                    marked_count += 1;
                }
            }
            if marked_count == 5 {
                return true;
            }
        }

        false
    }

    pub fn sum_of_unmarked(&self) -> u32 {
        let mut sum = 0;
        for row in self.spaces {
            for col in row {
                if !col.marked {
                    sum += col.value;
                }
            }
        }
        sum
    }

    pub fn mark_if_present(&mut self, value: u32) {
        for row in &mut self.spaces {
            for col in row {
                if col.value == value && !col.marked {
                    col.marked = true;
                }
            }
        }
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

fn run_game(vals: &[u32], boards: &mut [BingoBoard]) -> Option<(u32, usize)> {
    for i in 0..vals.len() {
        let val = vals[i];
        for board_idx in 0..boards.len() {
            let board = &mut boards[board_idx];
            board.mark_if_present(val);
            if i >= 4 && board.is_winner() {
                return Some((val, board_idx));
            }
        }
    }

    None
}

fn main() -> anyhow::Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let (values, mut bingo_boards) = load_input_data(&input_str);
    drop(input_str);

    let results = run_game(&values, &mut bingo_boards);

    match results {
        Some((value, idx)) => {
            let score = bingo_boards[idx].sum_of_unmarked() * value;
            println!("Board {} wins! Score: {score}", idx + 1);
        }
        None => {
            println!("No winning boards");
        }
    };

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
    fn test_is_winner() {
        let mut test_board = BINGO_BOARD.clone();

        assert_eq!(test_board.is_winner(), false);

        let marked_row = 4;
        for x in 0..5 {
            test_board.spaces[marked_row][x].marked = true;
        }

        assert_eq!(test_board.is_winner(), true);
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
        let expected_board_idx = 2;
        let expected_winning_val = 24;

        let winner = run_game(&values, &mut bingo_boards);
        assert!(winner.is_some());
        let (winning_val, board_idx) = winner.unwrap();
        assert_eq!(winning_val, expected_winning_val);
        assert_eq!(board_idx, expected_board_idx);
    }
}
