use crate::days::Challenge;
use std::fs;
use std::ops::Range;
use std::path::Path;

pub struct Day2<'a> {
    pub(crate) data: Option<Vec<(Range<usize>, String, String)>>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<usize>,
    pub(crate) part_2_ans: Option<usize>,
}

fn validate_passwords(
    passwords: &Vec<(Range<usize>, String, String)>,
    filter_fn: fn(&(Range<usize>, String, String)) -> bool,
) -> usize {
    passwords.iter().filter(|t| (filter_fn)(t)).count()
}

impl Challenge<'_> for Day2<'_> {
    fn setup(&mut self) {
        self.data = Some(
            fs::read_to_string(self.file_path)
                .expect("Could not read the input file")
                .lines()
                .map(|s| {
                    let tokens: Vec<&str> = s.split_whitespace().map(|s| s.trim()).collect();
                    assert_eq!(tokens.len(), 3);

                    let range_token: Vec<usize> = tokens[0]
                        .split("-")
                        .map(|s| {
                            usize::from_str_radix(s, 10).expect("Failed to parse integer value")
                        })
                        .collect();
                    assert_eq!(range_token.len(), 2);

                    let char_token = String::from(tokens[1].replace(":", ""));

                    (
                        Range {
                            start: range_token[0],
                            end: range_token[1] + 1,
                        },
                        char_token,
                        String::from(tokens[2]),
                    )
                })
                .collect(),
        );
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(validate_passwords(
            self.data.as_ref().unwrap(),
            |(r, c, p)| {
                let matches: Vec<_> = p.matches(c).collect();

                r.contains(&matches.len())
            },
        ));
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(validate_passwords(
            self.data.as_ref().unwrap(),
            |(r, c, p)| {
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
