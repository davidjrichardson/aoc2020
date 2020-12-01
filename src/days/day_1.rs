use crate::days::Challenge;

use itertools::Itertools;
use std::fs;

use crate::Path;

pub struct Day1<'a> {
    pub(crate) data: Option<Vec<i32>>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<Vec<i32>>,
    pub(crate) part_2_ans: Option<Vec<i32>>,
}

fn get_combinations(nums: &Vec<i32>, size: usize) -> Vec<i32> {
    nums.iter()
        .cloned()
        .combinations(size)
        .find(|v| v.iter().sum::<i32>() == 2020)
        .unwrap()
}

fn format_ans(val: &Option<Vec<i32>>) -> String {
    let unwrapped = val.as_ref().unwrap();

    format!(
        "Numbers: {:?}, answer: {:?}",
        unwrapped,
        unwrapped.iter().cloned().fold1(|x, y| x * y).unwrap()
    )
}

impl Challenge<'_> for Day1<'_> {
    fn setup(&mut self) {
        let data_raw: Vec<i32> = fs::read_to_string(self.file_path)
            .expect("Could not read the input file")
            .lines()
            .map(|s| i32::from_str_radix(s, 10).expect("Failed to parse integer value"))
            .sorted()
            .collect();

        self.data = Some(
            data_raw
                .iter()
                .cloned()
                .filter(|x| x + data_raw.get(0).unwrap() <= 2020)
                .collect(),
        );
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(get_combinations(&self.data.as_ref().unwrap(), 2));
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(get_combinations(&self.data.as_ref().unwrap(), 3));
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            format_ans(&self.part_1_ans),
            format_ans(&self.part_2_ans)
        )
    }
}
