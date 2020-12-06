use std::fs;
use std::path::Path;

use crate::days::Challenge;

pub struct Day6<'a> {
    data: Vec<Vec<u32>>,
    file_path: &'a Path,
    part_1_ans: Option<u32>,
    part_2_ans: Option<u32>,
}

impl<'a> Day6<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day6> {
        Box::new(Day6 {
            data: Vec::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

fn mask_union(group: &Vec<u32>) -> u32 {
    group.iter().fold(0, |acc, v| acc | v).count_ones()
}

fn mask_intersection(group: &Vec<u32>) -> u32 {
    group.iter().fold(u32::MAX, |acc, v| acc & v).count_ones()
}

impl Challenge<'_> for Day6<'_> {
    fn setup(&mut self) {
        let raw_str = fs::read_to_string(self.file_path).unwrap();

        self.data = raw_str
            .split("\n\n")
            .map(|c| {
                c.lines()
                    .map(|x| x.as_bytes().iter().map(|b| 1 << (b - 97u8)).sum())
                    .collect()
            })
            .collect();
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(self.data.iter().map(|g| mask_union(g)).sum());
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(self.data.iter().map(|g| mask_intersection(g)).sum());
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap_or(0),
            self.part_2_ans.unwrap_or(0)
        )
    }
}
