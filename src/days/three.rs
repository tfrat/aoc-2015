use crate::days::Day;
use std::collections::HashSet;

#[derive(Default)]
pub struct DayThree {}

impl DayThree {
    fn count_houses(directions: &str) -> u32 {
        directions
            .chars()
            .scan((0, 0), |pos, dir| {
                match dir {
                    '^' => pos.1 += 1,
                    'v' => pos.1 -= 1,
                    '>' => pos.0 += 1,
                    '<' => pos.0 -= 1,
                    _ => (),
                }
                Some(*pos)
            })
            .chain(std::iter::once((0, 0)))
            .collect::<HashSet<(i32, i32)>>()
            .len() as u32
    }
}

impl Day for DayThree {
    fn part_one(&self, input: &str) -> String {
        Self::count_houses(input).to_string()
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
        let day = DayThree::default();
        let cases = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayThree::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
