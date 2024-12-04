use crate::days::Day;

#[derive(Default)]
pub struct DayXXXX {}

impl DayXXXX {}

impl Day for DayXXXX {
    fn part_one(&self, input: &str) -> String {
        input
    }

    fn part_two(&self, input: &str) -> String {
        input
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayXXXX::default();
    }

    #[test]
    fn test_part_two() {
        let day = DayXXXX::default();
    }
}
