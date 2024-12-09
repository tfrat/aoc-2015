use crate::days::Day;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
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
    fn perform(instructions: &str, part_one: &bool) -> u32 {
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
            .par_bridge()
            .map(|(instruction, tl, br)| {
                let mut lights: HashMap<(i32, i32), i32> = HashMap::new();
                for y in tl.1..=br.1 {
                    for x in tl.0..=br.0 {
                        let new_coord = (x, y);
                        let light_level = lights.entry(new_coord).or_default();
                        match (&instruction, *light_level, part_one) {
                            (Instruction::TurnOn, _, true) | (Instruction::Toggle, 0, true) => {
                                *light_level = 1;
                            }
                            (Instruction::TurnOff, _, true) | (Instruction::Toggle, 1, true) => {
                                *light_level = 0;
                            }
                            (Instruction::TurnOn, _, false) => {
                                *light_level += 1;
                            }
                            (Instruction::TurnOff, _, false) => {
                                *light_level -= 1;
                            }
                            (Instruction::Toggle, _, false) => {
                                *light_level += 2;
                            }
                            _ => {}
                        }
                    }
                }
                lights
            })
            .reduce(HashMap::new, |mut acc, lights| {
                lights.iter().for_each(|(key, light)| {
                    acc.entry(*key)
                        .and_modify(|e| *e = (*e + light).max(0))
                        .or_insert(*light);
                });
                acc
            })
            .values()
            .map(|v| *v.max(&0) as u32)
            .sum()
    }
}

impl Day for DaySix {
    fn part_one(&self, input: &str) -> String {
        DaySix::perform(input, &true).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        // too low: 575856
        // too high: 15070510
        DaySix::perform(input, &false).to_string()
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
        let cases = vec![
            ("turn on 0,0 through 0,0", 1),
            ("toggle 0,0 through 99,99", 20_000),
            ("toggle 0,0 through 0,0\nturn off 0,0 through 0,0", 1),
            (
                "turn on 0,0 through 0,0\ntoggle 0,0 through 99,99\nturn off 0,0 through 4,4",
                19_976,
            ),
            ("turn on 0,0 through 0,0\ntoggle 0,0 through 99,99", 20_001),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
