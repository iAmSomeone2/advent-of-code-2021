use std::{
    error::Error,
    fmt::{self, Debug},
    fs,
};

struct DataError;

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input data is invalid")
    }
}

impl Debug for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for DataError {}

struct CrabSubs {
    /// Counts of the number of subs at each position
    sub_positions: Vec<u32>,
    max_position: u32,
    min_position: u32,
}

impl CrabSubs {
    pub fn new(positions: &[u32]) -> Option<Self> {
        if positions.len() == 0 {
            return None;
        }

        let max_position: u32 = match positions.iter().max() {
            Some(max) => *max,
            None => 0,
        };
        let min_position: u32 = match positions.iter().min() {
            Some(min) => *min,
            None => 0,
        };

        let mut sub_positions: Vec<u32> = vec![0; (max_position + 1) as usize];
        for position in positions {
            sub_positions[*position as usize] += 1;
        }

        Some(Self {
            sub_positions,
            max_position,
            min_position,
        })
    }

    /// Calculates the minimum amount of fuel required to align all positions
    ///
    /// Returns a tuple containing the position and fuel cost
    pub fn calculate_minimum_fuel_cost(&self) -> (usize, u64) {
        // For each position, get the fuel cost that each sub would have
        let mut min_fuel_cost: (usize, u64) = (0, u64::MAX);
        for i in 0..self.sub_positions.len() {
            let mut fuel_cost: u64 = 0;
            for j in 0..self.sub_positions.len() {
                let target_pos = i as i64;
                let current_pos = j as i64;
                let distance = (current_pos - target_pos).abs() as u64;
                let sub_count = self.sub_positions[j] as u64;
                fuel_cost += distance * sub_count;
            }
            if fuel_cost < min_fuel_cost.1 {
                min_fuel_cost = (i, fuel_cost);
            }
        }

        min_fuel_cost
    }

    /// Calculates the minimum amount of fuel required to align all positions according to the
    /// part 2 logic
    ///
    /// Returns a tuple containing the position and fuel cost
    pub fn calculate_minimum_fuel_cost_v2(&self) -> (usize, u64) {
        let mut min_fuel_cost: (usize, u64) = (0, u64::MAX);
        for i in 0..self.sub_positions.len() {
            let mut position_fuel_cost: u64 = 0;
            for j in 0..self.sub_positions.len() {
                let target_pos = i as i64;
                let current_pos = j as i64;
                let distance = (current_pos - target_pos).abs() as u64;
                let fuel_cost: u64 = (1..=distance).sum();
                let sub_count = self.sub_positions[j] as u64;
                position_fuel_cost += fuel_cost * sub_count;
            }
            if position_fuel_cost < min_fuel_cost.1 {
                min_fuel_cost = (i, position_fuel_cost);
            }
        }

        min_fuel_cost
    }
}

fn load_input_data(input: &str) -> Result<CrabSubs, DataError> {
    let positions: Vec<u32> = input
        .split(",")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    CrabSubs::new(&positions).ok_or(DataError)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let crab_subs = load_input_data(&input)?;
    drop(input);

    let results = crab_subs.calculate_minimum_fuel_cost();
    println!(
        "Part 1 Results:\n\tBest position:\t{}\n\tUsed fuel:\t{}",
        results.0, results.1
    );

    let results = crab_subs.calculate_minimum_fuel_cost_v2();
    println!(
        "\nPart 2 Results:\n\tBest position:\t{}\n\tUsed fuel:\t{}",
        results.0, results.1
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_POSITIONS: [u32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn new_crab_subs_test() {
        let expected_sub_positions = vec![1, 2, 3, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1];
        let expected_max = 16;
        let expected_min = 0;

        let crab_subs = CrabSubs::new(&TEST_POSITIONS);
        assert!(crab_subs.is_some());

        let crab_subs = crab_subs.unwrap();
        assert_eq!(crab_subs.max_position, expected_max);
        assert_eq!(crab_subs.min_position, expected_min);
        assert_eq!(crab_subs.sub_positions, expected_sub_positions);
    }

    #[test]
    fn calculate_minimum_fuel_cost_test() {
        let expected = (2, 37);

        let crab_subs = CrabSubs::new(&TEST_POSITIONS).unwrap();
        assert_eq!(crab_subs.calculate_minimum_fuel_cost(), expected);
    }

    #[test]
    fn calculate_minimum_fuel_cost_v2_test() {
        let expected = (5, 168);

        let crab_subs = CrabSubs::new(&TEST_POSITIONS).unwrap();
        assert_eq!(crab_subs.calculate_minimum_fuel_cost_v2(), expected);
    }
}
