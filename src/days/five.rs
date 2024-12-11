use crate::days::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

    fn count_nice_strings_v2(strings: &str) -> u32 {
        strings
            .lines()
            .filter(|line| {
                line.chars()
                    .enumerate()
                    .zip(line.chars().enumerate().skip(1))
                    .map(|((first_pos, first_letter), (second_pos, second_letter))| {
                        (
                            [first_letter, second_letter].iter().collect::<String>(),
                            HashSet::from([first_pos, second_pos]),
                        )
                    })
                    .scan(
                        HashMap::new(),
                        |pairs: &mut HashMap<String, HashSet<usize>>, (pair, indexes)| {
                            let pair_indexes = pairs.entry(pair).or_default();
                            if pair_indexes.intersection(&indexes).count() == 0 {
                                pair_indexes.extend(indexes);
                            }
                            Some(pair_indexes.len())
                        },
                    )
                    .any(|count| count >= 4)
            })
            .filter(|line| {
                line.chars()
                    .zip(strings.chars().skip(2))
                    .any(|(left, right)| left == right)
            })
            .count() as u32
    }
}

impl Day for DayFive {
    fn part_one(&self, input: &str) -> String {
        DayFive::count_nice_strings(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // incorrect: 59
        DayFive::count_nice_strings_v2(input).to_string()
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
        let cases = vec![
            ("qjhvhtzxzqqjkmpb", 1),
            ("xxyxx", 1),
            ("uurcxstgmygtbstg", 0),
            ("ieodomkazucvgmuy", 0),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
