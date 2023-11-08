#[macro_use]
extern crate lazy_static;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{error::Error, fs, ops::Range, path::PathBuf, sync::mpsc, thread};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    /// Simulate spawning for a given period and return the total number of fish
    pub fn simulate_spawning(&self, sim_time: usize) -> usize {
        let mut spawns = vec![self.clone()];
        for _ in 0..sim_time {
            let mut new_spawns: Vec<Lanternfish> = spawns
                .iter_mut()
                .map(|spawn| spawn.simulate_day())
                .filter(|new_spawn| new_spawn.is_some())
                .map(|new_spawn| new_spawn.unwrap())
                .collect();
            spawns.append(&mut new_spawns);
        }

        spawns.len()
    }

    fn simulate_spawning_group(
        lanternfish: &Vec<Lanternfish>,
        sim_time: usize,
        pb: Option<&ProgressBar>,
        total_pb: Option<&ProgressBar>,
    ) -> usize {
        match pb {
            Some(pb) => pb.set_length(lanternfish.len() as u64),
            None => {}
        }

        let mut count = 0;
        for fish in lanternfish {
            count += fish.simulate_spawning(sim_time);
            match pb {
                Some(pb) => pb.inc(1),
                None => {}
            }
        }
        count
    }
}

fn load_input_data(input: &str) -> Vec<Lanternfish> {
    input
        .split(",")
        .filter_map(|val| val.parse::<u8>().ok())
        .map(|timer| Lanternfish::new(timer))
        .collect()
}

fn compute_subvec_range(
    vec_size: usize,
    subvec_size: usize,
    iteration: usize,
    max_iter: usize,
) -> Range<usize> {
    let subvec_start = iteration * subvec_size;
    let subvec_end = if iteration < max_iter - 1 {
        subvec_start + subvec_size
    } else {
        vec_size
    };

    subvec_start..subvec_end
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

/*
    # Improving Performance

    - Split data set according to the number of available cores
    - Run simulations in parallel
    - Have each thread use a channel to return the final count
        - Use another channel to return processing status
    - Sum the final counts from all simulations
*/

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Load input data
    let input_data = fs::read_to_string(args.input_path)?;
    let lanternfish = load_input_data(&input_data);
    drop(input_data);

    // Determine how many threads can be used and how data should be split
    let thread_count = thread::available_parallelism()?;
    let subvec_size = lanternfish.len() / thread_count;

    // Set up progress indicators
    let multi_progress = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("#>-");
    let mut progress_bars = Vec::with_capacity(thread_count.get() + 1);
    for i in 0..=thread_count.get() {
        let pb = if i == thread_count.get() {
            let total_iters = args.simulation_time * thread_count.get();
            let pb = multi_progress.add(ProgressBar::new(total_iters as u64));
            pb.set_style(sty.clone());
            pb
        } else {
            let pb = multi_progress.add(ProgressBar::new(args.simulation_time as u64));
            pb.set_style(sty.clone());
            pb
        };
        progress_bars.push(pb);
    }

    // Set up channels and threads
    let (count_tx, count_rx) = mpsc::channel::<usize>();
    let mut children = Vec::with_capacity(thread_count.get());
    let mut fish_counts = vec![0];

    for id in 0..thread_count.get() {
        let simulation_time = args.simulation_time;
        let thread_count_tx = count_tx.clone();

        let subvec_range =
            compute_subvec_range(lanternfish.len(), subvec_size, id, thread_count.get());
        let mut subvec: Vec<Lanternfish> = subvec_range.map(|i| lanternfish[i]).collect();

        // Set up the thread's progress bar
        let thread_pb = progress_bars[id].clone();
        thread_pb.set_message(format!("thread {id}: simulating"));
        let total_pb = progress_bars[thread_count.get()].clone();

        let child = thread::spawn(move || {
            let count = Lanternfish::simulate_spawning_group(
                &mut subvec,
                simulation_time,
                Some(&thread_pb),
                Some(&total_pb),
            );
            thread_pb.finish_with_message(format!("thread {id}: done"));
            thread_count_tx.send(count).unwrap();
        });
        children.push(child);
    }
    drop(lanternfish);

    for _ in 0..children.len() {
        fish_counts.push(count_rx.recv().unwrap_or(0));
    }
    progress_bars[thread_count.get()].finish_with_message("all done");

    // Join all child threads
    for child in children {
        child.join().expect("Child thread panicked");
    }

    multi_progress.clear()?;

    let total: usize = fish_counts.iter().sum();
    println!("\nTotal: {total}");

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
    fn compute_subvec_range_test() {
        let vec_size = 9;
        let subvec_size = 2;
        let max_iter = 5;

        let expected = [0..2, 2..4, 4..6, 6..8, 8..9];

        for i in 0..max_iter {
            let computed_range = compute_subvec_range(vec_size, subvec_size, i, max_iter);
            assert_eq!(computed_range, expected[i]);
        }
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
    fn simulate_spawning_group_test() {
        let test_data = [(18, 26), (80, 5934)];

        for (sim_time, expected_count) in test_data {
            let test_fish = TEST_FISH.clone();
            let count = Lanternfish::simulate_spawning_group(&test_fish, sim_time, None, None);

            assert_eq!(count, expected_count);
        }
    }
}
