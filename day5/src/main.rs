use std::{fs, io, time::Instant};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq, Eq)]
struct VentLine {
    start: (i32, i32),
    end: (i32, i32),
    slope: (i32, i32),
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl VentLine {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let height_delta = end.1 - start.1;
        let horiz_delta = end.0 - start.0;

        let (x_min, x_max) = if start.0 <= end.0 {
            (start.0, end.0)
        } else {
            (end.0, start.0)
        };
        let (y_min, y_max) = if start.1 <= end.1 {
            (start.1, end.1)
        } else {
            (end.1, start.1)
        };

        Self {
            start,
            end,
            slope: (height_delta, horiz_delta),
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn intersects_with(&self, point: (i32, i32), include_angled: bool) -> bool {
        // Handle point being outside of line segment's range
        if point.0 < self.x_min
            || point.0 > self.x_max
            || point.1 < self.y_min
            || point.1 > self.y_max
        {
            return false;
        }

        // Handle vertical line
        if self.slope.1 == 0 {
            return point.0 == self.start.0;
        }

        // Handle horizontal line {
        if self.slope.0 == 0 {
            return point.1 == self.start.1;
        }

        if !include_angled {
            return false;
        }

        /*
            Start with: y - y1 = m(x - x1)
            End with y = mx + b
        */
        let (x1, y1) = self.start;
        let m = self.slope.0 as f32 / self.slope.1 as f32;
        let b = (-x1 as f32 * m) + y1 as f32;

        let y = m * (point.0 as f32) + b;

        // Check if the computed `y` points to the same cell as the given `y`
        y.floor() as i32 == point.1
    }
}

struct VentGrid {
    vent_lines: Vec<VentLine>,
    width: usize,
    height: usize,
}

impl VentGrid {
    pub fn new(vent_lines: Vec<VentLine>) -> Self {
        // Compute grid dimensions
        let width = vent_lines
            .iter()
            .max_by(|x, y| x.x_max.cmp(&y.x_max))
            .unwrap()
            .x_max as usize
            + 1;
        let height = vent_lines
            .iter()
            .max_by(|x, y| x.y_max.cmp(&y.y_max))
            .unwrap()
            .y_max as usize
            + 1;

        Self {
            vent_lines,
            width,
            height,
        }
    }

    pub fn calculate_coverage(&self, include_angled: bool) -> Vec<Vec<u32>> {
        let mut coverage = vec![vec![0; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let hits = self
                    .vent_lines
                    .iter()
                    .filter(|vent_line| {
                        vent_line.intersects_with((x as i32, y as i32), include_angled)
                    })
                    .count() as u32;
                coverage[y][x] = hits;
            }
        }

        coverage
    }
}

fn load_input_data(input: &str) -> Vec<VentLine> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<(i32, i32)> = line
                .split(" -> ")
                .take(2)
                .map(|point_str| {
                    let point = point_str
                        .split(",")
                        .take(2)
                        .filter_map(|val_str| val_str.parse::<i32>().ok())
                        .collect::<Vec<i32>>();
                    (point[0], point[1])
                })
                .collect();

            VentLine::new(coords[0], coords[1])
        })
        .collect()
}

fn calculate_danger_score(coverage_grid: &Vec<Vec<u32>>) -> usize {
    coverage_grid
        .iter()
        .flatten()
        .filter(|&value| *value >= 2)
        .count()
}

fn main() -> Result<(), io::Error> {
    println!("Loading input...");
    let mut start_time = Instant::now();
    let input = fs::read_to_string("input.txt")?;
    let vent_grid = VentGrid::new(load_input_data(&input));
    drop(input);
    let mut elapsed = Instant::now() - start_time;
    println!("done ({}ms)\n", elapsed.as_millis());

    // ==============================
    // Part 1 - No angles calculation
    // ==============================

    println!("Calculating coverage (no angles)...");
    start_time = Instant::now();
    let coverage_grid = vent_grid.calculate_coverage(false);
    elapsed = Instant::now() - start_time;
    println!("done ({}ms)\n", elapsed.as_millis());

    println!("Calculating danger score (no angles)...");
    start_time = Instant::now();
    let danger_score = calculate_danger_score(&coverage_grid);
    elapsed = Instant::now() - start_time;
    println!("done ({}ms)\n", elapsed.as_millis());

    println!("Danger score (no angles): {danger_score}\n\n");

    // ==============================
    // Part 2 - With angles calculation
    // ==============================

    println!("Calculating coverage (with angles)...");
    start_time = Instant::now();
    let coverage_grid = vent_grid.calculate_coverage(true);
    elapsed = Instant::now() - start_time;
    println!("done ({}ms)\n", elapsed.as_millis());

    println!("Calculating danger score (with angles)...");
    start_time = Instant::now();
    let danger_score = calculate_danger_score(&coverage_grid);
    elapsed = Instant::now() - start_time;
    println!("done ({}ms)\n", elapsed.as_millis());

    println!("Danger score (with angles): {danger_score}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref VERTICAL_VENT_LINE: VentLine = VentLine::new((2, 3), (2, 8));
        static ref HORIZONTAL_VENT_LINE: VentLine = VentLine::new((3, 2), (8, 2));
    }

    const TEST_INPUT_DATA_FULL: &'static str = "0,9 -> 5,9\n\
                                                8,0 -> 0,8\n\
                                                9,4 -> 3,4\n\
                                                2,2 -> 2,1\n\
                                                7,0 -> 7,4\n\
                                                6,4 -> 2,0\n\
                                                0,9 -> 2,9\n\
                                                3,4 -> 1,4\n\
                                                0,0 -> 8,8\n\
                                                5,5 -> 8,2";

    const TEST_INPUT_DATA_PARTIAL: &'static str = "0,9 -> 5,9";

    const COVERAGE_NO_ANGLES: [[u32; 10]; 10] = [
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
    ];

    const COVERAGE_WITH_ANGLES: [[u32; 10]; 10] = [
        [1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
        [0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
        [0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
        [0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
        [0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
        [0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
    ];

    #[test]
    fn test_new_vent_line() {
        let start = (2, 3);
        let end = (6, 4);
        let expected_slope = (1, 4);

        assert_eq!(
            VentLine::new(start, end),
            VentLine {
                start,
                end,
                slope: expected_slope,
                x_min: 2,
                x_max: 6,
                y_min: 3,
                y_max: 4
            }
        );
    }

    #[test]
    fn test_vertical_vent_line_intersects_with() {
        let test_data = [((1, 1), false), ((2, 6), true), ((2, 9), false)];

        for (point, expected) in test_data {
            assert_eq!(VERTICAL_VENT_LINE.intersects_with(point, false), expected);
        }
    }

    #[test]
    fn test_horizontal_vent_line_intersects_with() {
        let test_data = [((1, 1), false), ((5, 2), true), ((2, 9), false)];

        for (point, expected) in test_data {
            assert_eq!(HORIZONTAL_VENT_LINE.intersects_with(point, false), expected);
        }
    }

    #[test]
    fn test_vent_line_intersects_with() {
        let vent_line = VentLine::new((1, 6), (3, 2));
        let test_data = [((2, 4), true), ((2, 6), false), ((0, 7), false)];

        for (point, expected) in test_data {
            assert_eq!(
                vent_line.intersects_with(point, true),
                expected,
                "expected '{expected}' when point is ({}, {})",
                point.0,
                point.1
            );
        }
    }

    #[test]
    fn test_load_input_data() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_PARTIAL);

        assert_eq!(vent_lines.len(), 1);
        assert_eq!(vent_lines[0], VentLine::new((0, 9), (5, 9)));
    }

    #[test]
    fn test_new_vent_grid() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_FULL);
        let vent_grid = VentGrid::new(vent_lines);

        assert_eq!(vent_grid.width, 10);
        assert_eq!(vent_grid.height, 10);
    }

    #[test]
    fn test_vent_grid_coverage_no_angles() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_FULL);
        let vent_grid = VentGrid::new(vent_lines);

        let coverage = vent_grid.calculate_coverage(false);
        assert_eq!(coverage, COVERAGE_NO_ANGLES);
    }

    #[test]
    fn test_vent_grid_coverage_with_angles() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_FULL);
        let vent_grid = VentGrid::new(vent_lines);

        let coverage = vent_grid.calculate_coverage(true);
        assert_eq!(coverage, COVERAGE_WITH_ANGLES);
    }

    #[test]
    fn test_calculate_danger_score_no_angles() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_FULL);
        let vent_grid = VentGrid::new(vent_lines);

        let coverage = vent_grid.calculate_coverage(false);

        assert_eq!(calculate_danger_score(&coverage), 5);
    }

    #[test]
    fn test_calculate_danger_score_with_angles() {
        let vent_lines = load_input_data(TEST_INPUT_DATA_FULL);
        let vent_grid = VentGrid::new(vent_lines);

        let coverage = vent_grid.calculate_coverage(true);

        assert_eq!(calculate_danger_score(&coverage), 12);
    }
}
