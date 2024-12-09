use crate::Part;

mod five;
mod four;
mod one;
mod six;
mod three;
mod two;

pub trait Day {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_day(day: &u8, part: &Part) -> Result<Box<dyn Day>, String> {
    match (day, part) {
        (1, _) => Ok(Box::new(one::DayOne::default())),
        (2, _) => Ok(Box::new(two::DayTwo::default())),
        (3, _) => Ok(Box::new(three::DayThree::default())),
        (4, _) => Ok(Box::new(four::DayFour::default())),
        (5, _) => Ok(Box::new(five::DayFive::default())),
        (6, _) => Ok(Box::new(six::DaySix::default())),
        _ => Err(format!("Day {} not supported.", day)),
    }
}
