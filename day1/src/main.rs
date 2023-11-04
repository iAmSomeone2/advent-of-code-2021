use std::{fs, io};

fn load_test_data(path: &str) -> Result<Vec<i32>, io::Error> {
    let contents = fs::read_to_string(path)?;

    let mut test_data = vec![];
    for line in contents.lines() {
        test_data.push(line.parse().unwrap());
    }

    Ok(test_data)
}

fn create_data_windows(data: &[i32]) -> Vec<i32> {
    let mut windows = vec![];
    for i in 0..(data.len() - 2) {
        windows.push(data[i] + data[i + 1] + data[i + 2]);
    }

    windows
}

fn count_increases(depths: &[i32]) -> u32 {
    let mut increase_count = 0;

    for i in 1..depths.len() {
        if (depths[i] - depths[i - 1]) > 0 {
            increase_count += 1;
        }
    }

    return increase_count;
}

const TEST_DATA_PATH: &'static str = "test_data.txt";

fn main() -> Result<(), io::Error> {
    let test_data = load_test_data(TEST_DATA_PATH)?;
    let indivdual_count = count_increases(&test_data);

    let windows = create_data_windows(&test_data);
    let windows_count = count_increases(&windows);

    println!("Individual increases: {indivdual_count}");
    println!("Windowed increases: {windows_count}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const DEPTHS: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    const EXPECTED_INCREASES: u32 = 7;

    #[test]
    fn test_count_increases() {
        assert_eq!(count_increases(&DEPTHS), EXPECTED_INCREASES);
    }

    #[test]
    fn test_create_data_windows() {
        let windows = vec![607, 618, 618, 617, 647, 716, 769, 792];

        assert_eq!(create_data_windows(&DEPTHS), windows);
    }
}
