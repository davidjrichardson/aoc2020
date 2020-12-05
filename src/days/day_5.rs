use crate::days::Challenge;
use itertools::Itertools;
use std::{fs, path::Path};

pub struct Day5<'a> {
    pub(crate) data: Vec<usize>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<usize>,
    pub(crate) part_2_ans: Option<usize>,
}

impl<'a> Day5<'_> {
    pub fn build(file_path: &'a Path) -> Result<Day5, ()> {
        Ok(Day5 {
            data: Vec::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

fn seat_to_id((x, y): (usize, usize)) -> usize {
    (x * 8) + y
}

impl Challenge<'_> for Day5<'_> {
    fn setup(&mut self) {
        let binary_str = fs::read_to_string(self.file_path)
            .unwrap()
            .chars()
            .map(|c| match c {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                x => x,
            })
            .collect::<String>();

        self.data = binary_str
            .split_whitespace()
            .map(|s| {
                (
                    usize::from_str_radix(&s[0..7], 2).unwrap(),
                    usize::from_str_radix(&s[7..], 2).unwrap(),
                )
            })
            .map(seat_to_id)
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
