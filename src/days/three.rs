use crate::days::Day;
use std::collections::HashSet;

#[derive(Default)]
pub struct DayThree {}

impl DayThree {
    fn count_houses(directions: &str, movers: &u32) -> u32 {
        directions
            .chars()
            .enumerate()
            .scan(((0, 0), (0, 0)), |positions, (i, dir)| {
                match (dir, i as u32 % movers) {
                    ('^', 0) => positions.0 .1 += 1,
                    ('^', 1) => positions.1 .1 += 1,
                    ('v', 0) => positions.0 .1 -= 1,
                    ('v', 1) => positions.1 .1 -= 1,
                    ('>', 0) => positions.0 .0 += 1,
                    ('>', 1) => positions.1 .0 += 1,
                    ('<', 0) => positions.0 .0 -= 1,
                    ('<', 1) => positions.1 .0 -= 1,
                    _ => (),
                }
                match i as u32 % movers {
                    0 => Some(positions.0),
                    1 => Some(positions.1),
                    _ => None,
                }
            })
            .chain(std::iter::once((0, 0)))
            .collect::<HashSet<(i32, i32)>>()
            .len() as u32
    }
}

impl Day for DayThree {
    fn part_one(&self, input: &str) -> String {
        Self::count_houses(input, &1).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        Self::count_houses(input, &2).to_string()
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
        let cases = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
