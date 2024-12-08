use crate::days::Day;
use regex::Regex;

#[derive(Default)]
pub struct DayFive {}

impl DayFive {
    fn count_nice_strings(strings: &str) -> u32 {
        let vowels = Regex::new(r"[aeiou]").unwrap();
        let bad_strings = Regex::new(r"ab|cd|pq|xy").unwrap();
        strings
            .lines()
            .filter(|line| bad_strings.find_iter(line).count() == 0)
            .filter(|line| vowels.find_iter(line).count() >= 3)
            .filter(|line| line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b))
            .count() as u32
    }
}

impl Day for DayFive {
    fn part_one(&self, input: &str) -> String {
        // too low: 147
        DayFive::count_nice_strings(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayFive::default();
        let cases = vec![
            ("aeiouaeiouaeiou", 0),
            ("ugknbfddgicrmopn", 1),
            ("aaa", 1),
            ("jchzalrnumimnmhp", 0),
            ("haegwjzuvuyypxyu", 0),
            ("dvszwmarrgswjxmb", 0),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFive::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
