use std::path::Path;

use day_1::Day1;
use day_2::Day2;

pub mod day_1;
pub mod day_2;

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
        1 => Ok(Box::new(Day1 {
            data: None,
            file_path: file_path,
            part_1_ans: None,
            part_2_ans: None,
        })),
        2 => Ok(Box::new(Day2 {
            data: None,
            file_path: file_path,
            part_1_ans: None,
            part_2_ans: None,
        })),
        _ => Err("".to_string()),
    }
}
