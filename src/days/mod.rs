use std::path::Path;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;
use day_8::Day8;
use day_10::Day10;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_8;
pub mod day_10;

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
        1 => Ok(Day1::build(file_path)),
        2 => Ok(Day2::build(file_path)),
        3 => Ok(Day3::build(file_path)),
        4 => Ok(Day4::build(file_path)),
        5 => Ok(Day5::build(file_path)),
        6 => Ok(Day6::build(file_path)),
        8 => Ok(Day8::build(file_path)),
        10 => Ok(Day10::build(file_path)),
        _ => Err(format!("Day {} is not implemented", day)),
    }
}
