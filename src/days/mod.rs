use std::path::Path;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;

pub mod day_1;
pub mod day_2;
pub mod day_3;

pub trait Challenge<'a> {
    fn setup(&mut self);
    fn part_1(&mut self);
    fn part_2(&mut self);
    fn format_answers(&self) -> String;
}

pub fn challenge_from_day<'a>(
    day: &u32,
    file_path: &'a Path,
) -> Result<Box<dyn Challenge<'a> + 'a>, String> {
    match day {
        1 => Ok(Box::new(Day1::build(file_path).unwrap())),
        2 => Ok(Box::new(Day2::build(file_path).unwrap())),
        3 => Ok(Box::new(Day3::build(file_path).unwrap())),
        _ => Err(format!("Day {} is not implemented", day)),
    }
}
