use crate::days::Day;
use regex::Regex;
use std::collections::HashSet;

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    fn from_string(input: &str) -> Option<Self> {
        match input {
            "turn off" => Some(Instruction::TurnOff),
            "turn on" => Some(Instruction::TurnOn),
            "toggle" => Some(Instruction::Toggle),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct DaySix {}

impl DaySix {
    fn perform(instructions: &str) -> u32 {
        let re = Regex::new(r"([a-z\s]+)(\d+),(\d+)[a-z\s]+(\d+),(\d+)").unwrap();
        instructions
            .lines()
            .map(|line| {
                let caps = re.captures(line).unwrap();
                let inst = Instruction::from_string(caps[1].trim()).unwrap();
                let tl: (i32, i32) = (caps[2].parse().unwrap(), caps[3].parse().unwrap());
                let br: (i32, i32) = (caps[4].parse().unwrap(), caps[5].parse().unwrap());
                (inst, tl, br)
            })
            .fold(HashSet::new(), |mut lights, (instruction, tl, br)| {
                for y in tl.1..=br.1 {
                    for x in tl.0..=br.0 {
                        let new_coord = (x, y);
                        match (&instruction, lights.get(&new_coord)) {
                            (Instruction::TurnOn, _) => {
                                lights.insert(new_coord);
                            }
                            (Instruction::TurnOff, _) => {
                                lights.remove(&new_coord);
                            }
                            (Instruction::Toggle, Some(_)) => {
                                lights.remove(&new_coord);
                            }
                            (Instruction::Toggle, None) => {
                                lights.insert(new_coord);
                            }
                        }
                    }
                }
                lights
            })
            .len() as u32
    }
}

impl Day for DaySix {
    fn part_one(&self, input: &str) -> String {
        DaySix::perform(input).to_string()
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
        let day = DaySix::default();
        let cases = vec![
            ("turn on 0,0 through 999,999", 1000 * 1000),
            ("toggle 0,0 through 999,0", 1000),
            ("turn off 499,499 through 500,500", 0),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DaySix::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
