use crate::days::Challenge;
use regex::Regex;
use std::{fs, path::Path, str::FromStr};

#[derive(Debug)]
enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColour::Amber),
            "blu" => Ok(EyeColour::Blue),
            "brn" => Ok(EyeColour::Brown),
            "gry" => Ok(EyeColour::Grey),
            "grn" => Ok(EyeColour::Green),
            "hzl" => Ok(EyeColour::Hazel),
            "oth" => Ok(EyeColour::Other),
            _ => Err(format!("Eye colour \"{}\" isn't valid", s)),
        }
    }
}

pub struct Day4<'a> {
    pub(crate) data: Option<Vec<Passport>>,
    pub(crate) file_path: &'a Path,
    pub(crate) part_1_ans: Option<usize>,
    pub(crate) part_2_ans: Option<usize>,
}

impl<'a> Day4<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day4> {
        Box::new(Day4 {
            data: None,
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

#[derive(Debug)]
struct Height {
    size: u32,
    unit: String,
}

impl Height {
    fn is_valid(&self) -> bool {
        let u = self.unit.as_str();

        match u {
            "in" => (59..77).contains(&self.size),
            "cm" => (150..194).contains(&self.size),
            _ => false,
        }
    }
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<size>[0-9]+)(?P<unit>(cm|in))").unwrap();
        }

        RE.captures(s)
            .map_or_else(|| None, |c| Some(c))
            .map_or_else(
                || None,
                |c| {
                    Some((
                        c.name("size").unwrap().as_str(),
                        c.name("unit").unwrap().as_str(),
                    ))
                },
            )
            .map_or_else(|| None, |(s, u)| Some((u32::from_str(s).unwrap(), u)))
            .map_or_else(
                || Err("Failed to parse height".to_string()),
                |(size, unit)| {
                    Ok(Height {
                        size,
                        unit: unit.to_string(),
                    })
                },
            )
    }
}

#[derive(Debug)]
pub struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

#[derive(Debug)]
pub struct StrongPassport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<EyeColour>,
    pid: Option<u64>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

impl StrongPassport {
    fn from_passport(passport: &Passport) -> StrongPassport {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"(?P<val>#[0-9a-z]{6})").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^(?P<val>[0-9]{9})$").unwrap();
        }

        StrongPassport {
            byr: passport.byr.and_then(|v| {
                if (1920..2003).contains(&v) {
                    Some(v)
                } else {
                    None
                }
            }),
            iyr: passport.iyr.and_then(|v| {
                if (2010..2021).contains(&v) {
                    Some(v)
                } else {
                    None
                }
            }),
            eyr: passport.eyr.and_then(|v| {
                if (2020..2031).contains(&v) {
                    Some(v)
                } else {
                    None
                }
            }),
            hgt: passport
                .hgt
                .as_ref()
                .and_then(|s| match Height::from_str(s.as_str()) {
                    Ok(h) => Some(h),
                    Err(_) => None,
                }),
            hcl: passport.hcl.as_ref().and_then(|s| {
                if HCL_RE.is_match(s.as_str()) {
                    Some(String::from(s))
                } else {
                    None
                }
            }),
            ecl: passport
                .ecl
                .as_ref()
                .and_then(|s| match EyeColour::from_str(s.as_str()) {
                    Ok(e) => Some(e),
                    Err(_) => None,
                }),
            pid: passport.pid.as_ref().and_then(|s| {
                if PID_RE.is_match(s.as_str()) {
                    Some(u64::from_str(s.as_str()).unwrap())
                } else {
                    None
                }
            }),
        }
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self
                .hgt
                .as_ref()
                .and_then(|h| if h.is_valid() { Some(h) } else { None })
                .is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

fn capture_as_type<T: FromStr>(regex: &Regex, input_str: &str) -> Option<T> {
    regex
        .captures(input_str)
        .map_or_else(|| None, |c| Some(c))
        .map_or_else(
            || None,
            |c| c.name("val").map_or_else(|| None, |m| Some(m.as_str())),
        )
        .map_or_else(
            || None,
            |s| match T::from_str(s) {
                Ok(a) => Some(a),
                _ => unreachable!(),
            },
        )
}

impl FromStr for Passport {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let token_str = s.replace("\n", " ");

        lazy_static! {
            static ref BYR_RE: Regex = Regex::new(r"byr:(?P<val>[0-9]+)").unwrap();
            static ref IYR_RE: Regex = Regex::new(r"iyr:(?P<val>[0-9]+)").unwrap();
            static ref EYR_RE: Regex = Regex::new(r"eyr:(?P<val>[0-9]+)").unwrap();
            static ref HGT_RE: Regex = Regex::new(r"hgt:(?P<val>\w+)").unwrap();
            static ref HCL_RE: Regex = Regex::new(r"hcl:(?P<val>[#0-9a-z]+)").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"ecl:(?P<val>[#0-9a-z]+)").unwrap();
            static ref PID_RE: Regex = Regex::new(r"pid:(?P<val>[#0-9a-z]+)").unwrap();
        }

        Ok(Passport {
            byr: capture_as_type(&BYR_RE, token_str.as_str()),
            iyr: capture_as_type(&IYR_RE, token_str.as_str()),
            eyr: capture_as_type(&EYR_RE, token_str.as_str()),
            hgt: capture_as_type(&HGT_RE, token_str.as_str()),
            hcl: capture_as_type(&HCL_RE, token_str.as_str()),
            ecl: capture_as_type(&ECL_RE, token_str.as_str()),
            pid: capture_as_type(&PID_RE, token_str.as_str()),
        })
    }
}

impl Challenge<'_> for Day4<'_> {
    fn setup(&mut self) {
        self.data = Some(
            fs::read_to_string(self.file_path)
                .unwrap()
                .split("\n\n")
                .map(|s| Passport::from_str(s).unwrap())
                .collect(),
        );
    }

    fn part_1(&mut self) {
        self.part_1_ans = Some(
            self.data
                .as_ref()
                .unwrap()
                .iter()
                .filter(|p| p.is_valid())
                .count(),
        );
    }

    fn part_2(&mut self) {
        self.part_2_ans = Some(
            self.data
                .as_ref()
                .unwrap()
                .iter()
                .map(|p| StrongPassport::from_passport(&p))
                .filter(|p| p.is_valid())
                .count(),
        );
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap(),
            self.part_2_ans.unwrap()
        )
    }
}
