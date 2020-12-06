use std::path::Path;
use std::{collections::HashSet, fs};

use crate::days::Challenge;

type DeclForm = Vec<HashSet<char>>;

pub struct Day6<'a> {
    pub(crate) data: Vec<DeclForm>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<usize>,
    pub(crate) part_2_ans: Option<usize>,
}

impl<'a> Day6<'_> {
    pub fn build(file_path: &'a Path) -> Result<Day6, ()> {
        Ok(Day6 {
            data: Vec::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

impl Challenge<'_> for Day6<'_> {
    fn setup(&mut self) {
        let raw_str = fs::read_to_string(self.file_path).unwrap();

        self.data = raw_str
            .split("\n\n")
            .map(|c| c.split_whitespace().map(|x| x.chars().collect()).collect())
            .collect();
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(self.data.iter().fold(0, |acc, v| {
            acc + v
                .iter()
                .fold(HashSet::new(), |a, s| a.union(s).cloned().collect())
                .len()
        }));
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(self.data.iter().fold(0, |acc, v| {
            acc + v
                .iter()
                .fold(v.get(0).unwrap().clone(), |a, s| {
                    a.intersection(s).cloned().collect()
                })
                .len()
        }));
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap_or(0),
            self.part_2_ans.unwrap_or(0)
        )
    }
}
