use crate::days::Day;

#[derive(Default)]
pub struct DayOne {}

impl DayOne {
    fn find_floor(floors: &str) -> i32 {
        floors
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum()
    }

    fn find_basement(floors: &str) -> usize {
        floors
            .chars()
            .enumerate()
            .scan(0, |sum, (i, ch)| {
                match ch {
                    '(' => *sum += 1,
                    ')' => *sum -= 1,
                    _ => (),
                }
                Some((i, *sum))
            })
            .find(|&(_, sum)| sum < 0)
            .map(|(i, _)| i + 1)
            .unwrap_or(0)
    }
}

impl Day for DayOne {
    fn part_one(&self, input: &str) -> String {
        let floor = Self::find_floor(input);
        floor.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let floor = Self::find_basement(input);
        floor.to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayOne::default();
        let cases = vec![("(())", 0), ("(((", 3), ("))(((((", 3)];
        for (input, result) in cases {
            assert_eq!(day.part_one(input), result.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayOne::default();
        let cases = vec![(")", 1), ("()())", 5)];
        for (input, result) in cases {
            assert_eq!(day.part_two(input), result.to_string())
        }
    }
}
