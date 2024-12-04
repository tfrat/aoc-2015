use crate::days::Day;
use std::cmp::min;

#[derive(Default)]
pub struct DayTwo {}

impl DayTwo {
    fn calculate_surface_area(measurements: &str) -> i32 {
        let dimensions: Vec<i32> = measurements
            .split("x")
            .map(|x| x.parse().unwrap())
            .collect();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        let one = l * w;
        let two = w * h;
        let three = l * h;

        2 * one + 2 * two + 2 * three + min(min(one, two), three)
    }

    fn calculate_ribbon_length(measurements: &str) -> i32 {
        let dimensions: Vec<i32> = measurements
            .split("x")
            .map(|x| x.parse().unwrap())
            .collect();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        let ribbon_length = min(min(2 * l + 2 * w, 2 * l + 2 * h), 2 * w + 2 * h);
        let bow_length = l * w * h;

        ribbon_length + bow_length
    }
}

impl Day for DayTwo {
    fn part_one(&self, input: &str) -> String {
        input
            .lines()
            .map(Self::calculate_surface_area)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input
            .lines()
            .map(Self::calculate_ribbon_length)
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayTwo::default();
        let cases = vec![("2x3x4", 58), ("1x1x10", 43)];
        for (input, result) in cases {
            assert_eq!(day.part_one(input), result.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayTwo::default();
        let cases = vec![("2x3x4", 34), ("1x1x10", 14)];
        for (input, result) in cases {
            assert_eq!(day.part_two(input), result.to_string())
        }
    }
}
