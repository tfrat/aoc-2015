use crate::days::Day;

#[derive(Default)]
pub struct DayFour {}

impl DayFour {
    fn find_hash(secret: &str, hash_prefix: &str) -> u32 {
        (0..)
            .find(|suffix| {
                format!(
                    "{:x}",
                    md5::compute(secret.to_string() + &*suffix.to_string())
                )
                .starts_with(hash_prefix)
            })
            .unwrap()
    }
}

impl Day for DayFour {
    fn part_one(&self, input: &str) -> String {
        DayFour::find_hash(input, "00000").to_string()
    }

    fn part_two(&self, input: &str) -> String {
        DayFour::find_hash(input, "000000").to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayFour::default();
        let cases = vec![("abcdef", 609043), ("pqrstuv", 1048970)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFour::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
