use crate::days::Challenge;
use std::ops::Range;
use std::path::Path;
use std::{fs, str::FromStr};

pub struct PasswordDetails(Range<usize>, String, String);

impl FromStr for PasswordDetails {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().map(|s| s.trim()).collect();
        let range_token: Vec<usize> = tokens[0]
            .split("-")
            .map(|s| usize::from_str_radix(s, 10).expect("Failed to parse integer value"))
            .collect();

        let char_token = String::from(tokens[1].replace(":", ""));
        Ok(PasswordDetails(
            Range {
                start: range_token[0],
                end: range_token[1] + 1,
            },
            char_token,
            String::from(tokens[2]),
        ))
    }
}

pub struct Day2<'a> {
    data: Option<Vec<PasswordDetails>>,
    file_path: &'a Path,
    part_1_ans: Option<usize>,
    part_2_ans: Option<usize>,
}

impl<'a> Day2<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day2> {
        Box::new(Day2 {
            data: None,
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path
        })
    }
}

fn validate_passwords(
    passwords: &Vec<PasswordDetails>,
    filter_fn: impl Fn(&PasswordDetails) -> bool,
) -> usize {
    passwords.iter().filter(|t| filter_fn(t)).count()
}

impl Challenge<'_> for Day2<'_> {
    fn setup(&mut self) {
        self.data = Some(
            fs::read_to_string(self.file_path)
                .expect("Could not read the input file")
                .lines()
                .map(|s| PasswordDetails::from_str(s).unwrap())
                .collect(),
        );
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(validate_passwords(
            self.data.as_ref().unwrap(),
            |PasswordDetails(r, c, p)| r.contains(&p.matches(c).count()),
        ));
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(validate_passwords(
            self.data.as_ref().unwrap(),
            |PasswordDetails(r, c, p)| {
                (p.get((r.start - 1)..(r.start)).unwrap() == c)
                    ^ (p.get((r.end - 2)..(r.end - 1)).unwrap() == c)
            },
        ));
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap(),
            self.part_2_ans.unwrap()
        )
    }
}
