use crate::Part;

mod one;
mod two;

pub trait Day {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_day(day: &u8, part: &Part) -> Result<Box<dyn Day>, String> {
    match (day, part) {
        (1, _) => Ok(Box::new(one::DayOne::default())),
        (2, _) => Ok(Box::new(two::DayTwo::default())),
        _ => Err(format!("Day {} not supported.", day)),
    }
}
