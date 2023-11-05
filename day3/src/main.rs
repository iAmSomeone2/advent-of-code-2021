#[macro_use]
extern crate lazy_static;
use std::fs;

struct DiagnosticReport {
    values: Vec<u16>,
    bit_width: usize,
}

impl DiagnosticReport {
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
        Ok(DiagnosticReport { values, bit_width })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RatingType {
    OxygenGen,
    CO2Scrub,
}

/// Calculates the most or least common bit at a given position.
///
/// Setting `rating_type` to `None` or `Some(RatingType::OxygenGen)` will calculate the most common bit; favoring 1 over 0.
/// Setting `rating_type` to `Some(RatingType::CO2Scrub)` will calculate the least common bit; favoring 0 over 1.
fn common_bit_at_position(
    values: (&[u16], usize),
    position: usize,
    rating_type: Option<RatingType>,
) -> u16 {
    let (values, bit_width) = values;
    let shift_step = bit_width - 1 - position;
    let mut count = (0, 0);
    for value in values {
        let bit = (*value >> shift_step) & 0x01;
        if bit == 0 {
            count.0 += 1;
        } else {
            count.1 += 1;
        }
    }

    return match rating_type {
        Some(rating_type) => match rating_type {
            RatingType::OxygenGen => {
                if count.1 >= count.0 {
                    1
                } else {
                    0
                }
            }
            RatingType::CO2Scrub => {
                if count.0 <= count.1 {
                    0
                } else {
                    1
                }
            }
        },
        None => {
            if count.1 >= count.0 {
                1
            } else {
                0
            }
        }
    };
}

struct Ratings {
    diagnostic_report: DiagnosticReport,
    gamma: u16,
    epsilon: u16,
}

impl Ratings {
    pub fn new(diagnostic_report: DiagnosticReport) -> Self {
        let most_common_bits = Ratings::calculate_most_common_bits(&diagnostic_report);
        let gamma = Ratings::get_gamma(&most_common_bits);
        let epsilon = Ratings::get_epsilon(&most_common_bits);
        Self {
            diagnostic_report,
            gamma,
            epsilon,
        }
    }

    fn calculate_most_common_bits(diagnostic_report: &DiagnosticReport) -> Vec<u16> {
        let mut bits = vec![0; diagnostic_report.bit_width];
        for i in 0..diagnostic_report.bit_width {
            bits[i] = common_bit_at_position(
                (&diagnostic_report.values, diagnostic_report.bit_width),
                i,
                None,
            );
        }

        bits
    }

    fn get_gamma(most_common_bits: &[u16]) -> u16 {
        let mut gamma = 0u16;
        let bit_width = most_common_bits.len();
        for i in 0..bit_width {
            gamma |= most_common_bits[i] << (bit_width - 1 - i);
        }

        gamma
    }

    fn get_epsilon(most_common_bits: &[u16]) -> u16 {
        let mut epsilon = 0u16;
        let bit_width = most_common_bits.len();
        for i in 0..bit_width {
            let bit = if most_common_bits[i] == 1 { 0 } else { 1 };
            epsilon |= bit << (bit_width - 1 - i);
        }

        epsilon
    }

    pub fn calculate_power_consumption(&self) -> u32 {
        self.gamma as u32 * self.epsilon as u32
    }

    fn get_rating(&self, rating_type: RatingType) -> u16 {
        let bit_width = self.diagnostic_report.bit_width;
        let mut values = vec![];
        for i in 0..bit_width {
            if values.len() == 1 {
                break;
            }

            let src_vec = if i == 0 {
                &self.diagnostic_report.values
            } else {
                &values
            };

            let test_bit =
                common_bit_at_position((src_vec, bit_width), i, Some(rating_type.clone()));
            let shift_steps = bit_width - 1 - i;
            values = src_vec
                .iter()
                .filter(move |&&x| {
                    let x_bit = (x >> shift_steps) & 0x01;
                    x_bit == test_bit
                })
                .map(|x| *x)
                .collect();
        }

        values[0]
    }
}

fn main() -> anyhow::Result<()> {
    let input_str = fs::read_to_string("input.txt")?;
    let diag_report = DiagnosticReport::from_str(&input_str)?;
    drop(input_str);
    let ratings = Ratings::new(diag_report);

    let power_consumption = ratings.calculate_power_consumption();

    println!("Power consumption: {power_consumption}");

    let oxygen_gen_rating = ratings.get_rating(RatingType::OxygenGen) as u32;
    let co2_scrub_rating = ratings.get_rating(RatingType::CO2Scrub) as u32;

    let life_support_rating = oxygen_gen_rating * co2_scrub_rating;
    println!("Life support rating: {life_support_rating}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref DIAG_VALUES: Vec<u16> = vec![
            0b00100u16, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010
        ];
        static ref MOST_COMMON_BITS: Vec<u16> = vec![1, 0, 1, 1, 0];
    }

    #[test]
    fn test_common_bit_at_position() {
        let test_data = [
            (0, None, 1),
            (1, Some(RatingType::OxygenGen), 0),
            (2, Some(RatingType::CO2Scrub), 0),
        ];

        for (position, rating_type, expected) in test_data {
            assert_eq!(
                common_bit_at_position((DIAG_VALUES.as_ref(), 5), position, rating_type),
                expected
            );
        }
    }

    #[test]
    fn test_input_data_from_str() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        let expected_count = 12;
        let expected_bit_width = 5;

        let input_data = DiagnosticReport::from_str(data).unwrap();
        assert_eq!(input_data.values.len(), expected_count);
        assert_eq!(input_data.bit_width, expected_bit_width);
    }

    #[test]
    fn test_calculate_most_common_bits() {
        assert_eq!(
            Ratings::calculate_most_common_bits(&DiagnosticReport {
                values: DIAG_VALUES.clone(),
                bit_width: 5,
            }),
            *MOST_COMMON_BITS
        );
    }

    #[test]
    fn test_get_gamma() {
        let expected = 0b10110;

        assert_eq!(Ratings::get_gamma(MOST_COMMON_BITS.as_ref()), expected);
    }

    #[test]
    fn test_get_epsilon() {
        let expected = 0b01001;

        assert_eq!(Ratings::get_epsilon(MOST_COMMON_BITS.as_ref()), expected);
    }

    #[test]
    fn test_get_rating() {
        let ratings = Ratings::new(DiagnosticReport {
            values: DIAG_VALUES.clone(),
            bit_width: 5,
        });
        let test_data = [(RatingType::OxygenGen, 23), (RatingType::CO2Scrub, 10)];

        for (rating_type, expected_rating) in test_data {
            let oxygen_gen_rating = ratings.get_rating(rating_type);
            assert_eq!(oxygen_gen_rating, expected_rating);
        }
    }
}
