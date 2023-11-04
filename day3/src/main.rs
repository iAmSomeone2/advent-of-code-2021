use std::fs;

struct InputData {
    values: Vec<u16>,
    bit_width: usize,
}

impl InputData {
    fn from_str(data: &str) -> anyhow::Result<Self> {
        let mut bit_width = 0;
        let mut values = vec![];
        for line in data.lines() {
            if line.chars().count() == 0 {
                continue;
            }
            if bit_width == 0 {
                bit_width = line.chars().count();
            }
            let value: u16 = u16::from_str_radix(line, 2)?;
            values.push(value);
        }
        Ok(InputData { values, bit_width })
    }

    fn calculate_gamma_and_epsilon(&self) -> (u16, u16) {
        let mut result = (0, 0);
        for i in 0..self.bit_width {
            let mut count = (0, 0);
            for value in &self.values {
                let bit = (*value >> i) & 0x01;
                if bit == 0 {
                    count.0 += 1;
                } else {
                    count.1 += 1;
                }
            }
            if count.0 > count.1 {
                result.0 = result.0 | (0 << i);
                result.1 = result.1 | (1 << i);
            } else {
                result.0 = result.0 | (1 << i);
                result.1 = result.1 | (0 << i);
            };
        }

        result
    }
}

fn calculate_power_consumption(gamma: u16, epsilon: u16) -> u32 {
    gamma as u32 * epsilon as u32
}

fn main() -> anyhow::Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let input_data = InputData::from_str(&input_str)?;
    drop(input_str);

    let (gamma, epsilon) = input_data.calculate_gamma_and_epsilon();
    let power_consumption = calculate_power_consumption(gamma, epsilon);

    println!("Power consumption: {power_consumption}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_gamma_and_epsilon() {
        let input_data = InputData {
            values: vec![
                0b00100u16, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ],
            bit_width: 5,
        };

        let expected_gamma = 0b10110;
        let expected_epsilon = 0b01001;

        assert_eq!(
            input_data.calculate_gamma_and_epsilon(),
            (expected_gamma, expected_epsilon)
        );
    }

    #[test]
    fn test_input_data_from_str() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        let expected_count = 12;
        let expected_bit_width = 5;

        let input_data = InputData::from_str(data).unwrap();
        assert_eq!(input_data.values.len(), expected_count);
        assert_eq!(input_data.bit_width, expected_bit_width);
    }
}
