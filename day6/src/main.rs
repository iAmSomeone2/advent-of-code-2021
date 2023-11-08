use std::{fs, io, path::PathBuf, time::Instant};

use clap::Parser;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lanternfish {
    spawn_timer: u8,
}

impl Lanternfish {
    const DEFAULT_SPAWN_TIMER: u8 = 6;
    const FIRST_SPAWN_TIMER: u8 = 8;

    pub fn new(spawn_timer: u8) -> Self {
        Self { spawn_timer }
    }

    /// Simulates a day for the Lanternfish
    ///
    /// 1. Check if `spawn_timer` is 0
    ///     1. If 0, reset to Lanternfish::DEFAULT_SPAWN_TIMER
    ///     2. Create and return new Lanternfish with Lanternfish::FIRST_SPAWN_TIMER
    /// 2. If > 0, decrement `spawn_timer` and return
    pub fn simulate_day(&mut self) -> Option<Self> {
        if self.spawn_timer == 0 {
            self.spawn_timer = Lanternfish::DEFAULT_SPAWN_TIMER;
            Some(Lanternfish::new(Lanternfish::FIRST_SPAWN_TIMER))
        } else {
            self.spawn_timer -= 1;
            None
        }
    }

    fn simulate_spawning(lanternfish: &mut Vec<Lanternfish>, simulation_time: usize) {
        for _ in 0..simulation_time {
            for i in 0..lanternfish.len() {
                let fish = &mut lanternfish[i];
                match fish.simulate_day() {
                    Some(new_fish) => lanternfish.push(new_fish),
                    None => {}
                }
            }
        }
    }
}

fn load_input_data(input: &str) -> Vec<Lanternfish> {
    input
        .split(",")
        .filter_map(|val| val.parse::<u8>().ok())
        .map(|timer| Lanternfish::new(timer))
        .collect()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to an input file
    #[arg(short, long, value_name = "input", default_value = "input.txt")]
    input_path: PathBuf,

    /// Number of days to simulate
    #[arg(value_name = "DAYS")]
    simulation_time: usize,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let input_data = fs::read_to_string(args.input_path)?;
    let mut lanternfish = load_input_data(&input_data);
    drop(input_data);

    println!("Simulating lanternfish spawning...");
    let start_time = Instant::now();
    Lanternfish::simulate_spawning(&mut lanternfish, args.simulation_time);
    let run_duration = Instant::now() - start_time;
    println!("done ({}ms)\n", run_duration.as_millis());

    println!(
        "Results:\n\tLanternfish count:\t{}\n\tSim time:\t\t{} days",
        lanternfish.len(),
        args.simulation_time
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref TEST_FISH: Vec<Lanternfish> = vec![
            Lanternfish::new(3),
            Lanternfish::new(4),
            Lanternfish::new(3),
            Lanternfish::new(1),
            Lanternfish::new(2)
        ];
    }

    const TEST_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn load_input_data_test() {
        assert_eq!(load_input_data(TEST_INPUT), *TEST_FISH);
    }

    #[test]
    fn simulate_day_test() {
        let test_data = [
            (Lanternfish::new(6), 5, None),
            (Lanternfish::new(0), 6, Some(Lanternfish::new(8))),
        ];

        for (mut input_fish, expected_timer, output_fish) in test_data {
            let output = input_fish.simulate_day();
            assert_eq!(
                input_fish.spawn_timer, expected_timer,
                "Actual timer {} does not match expected {}.",
                input_fish.spawn_timer, expected_timer
            );
            assert_eq!(
                output, output_fish,
                "Actual output {output:?} does not match expected {output_fish:?}"
            );
        }
    }

    #[test]
    fn simulate_spawning_test() {
        let test_data = [(18, 26), (80, 5934)];

        for (sim_time, expected_count) in test_data {
            let mut test_fish = TEST_FISH.clone();
            Lanternfish::simulate_spawning(&mut test_fish, sim_time);
            assert_eq!(test_fish.len(), expected_count);
        }
    }
}
