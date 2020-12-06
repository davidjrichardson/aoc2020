use crate::days::Challenge;
use itertools::Itertools;
use std::{fs, path::Path};

pub struct Day5<'a> {
    data: Vec<u16>,
    file_path: &'a Path,
    part_1_ans: Option<u16>,
    part_2_ans: Option<u16>,
}

impl<'a> Day5<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day5> {
        Box::new(Day5 {
            data: Vec::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

impl Challenge<'_> for Day5<'_> {
    fn setup(&mut self) {
        let input_strs = fs::read_to_string(self.file_path).unwrap();

        self.data = input_strs
            .split_whitespace()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        'B' | 'R' => 1u16,
                        'F' | 'L' => 0u16,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, x| (acc << 1) + x)
            })
            .sorted()
            .collect();
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(self.data.last().unwrap().to_owned());
    }

    fn part_2(&mut self) {
        self.part_2_ans = (0..self.data.len() - 2)
            .map(|i| (self.data.get(i).unwrap(), self.data.get(i + 1).unwrap()))
            .find(|(x, y)| (*x + 1) != **y)
            .map(|(x, _)| x + 1);
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap_or(0),
            self.part_2_ans.unwrap_or(0)
        )
    }
}
