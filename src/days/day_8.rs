use std::{collections::HashMap, fs, str::FromStr};
use std::{collections::HashSet, path::Path};

use crate::days::Challenge;

pub struct Day8<'a> {
    data: HashMap<u32, Instruction>,
    file_path: &'a Path,
    part_1_ans: Option<i32>,
    part_2_ans: Option<i32>,
}

impl<'a> Day8<'_> {
    pub fn build(file_path: &'a Path) -> Box<Day8> {
        Box::new(Day8 {
            data: HashMap::new(),
            part_1_ans: None,
            part_2_ans: None,
            file_path: file_path,
        })
    }
}

#[derive(Debug)]
enum Instruction {
    Jump(i32),
    Accumulate(i32),
    NoOp(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_ascii_whitespace().collect::<Vec<&str>>()[..2] {
            [ins, val] => match ins {
                "acc" => Ok(Instruction::Accumulate(
                    i32::from_str_radix(val, 10).unwrap(),
                )),
                "jmp" => Ok(Instruction::Jump(i32::from_str_radix(val, 10).unwrap())),
                "nop" => Ok(Instruction::NoOp(i32::from_str_radix(val, 10).unwrap())),
                _ => Err(format!(
                    "Invalid unary instruction type \"{}\" with value \"{}\" from str {}",
                    ins, val, s
                )),
            },
            _ => Err(format!("Could not parse instruction string \"{}\"", s)),
        }
    }
}

fn safe_add(lhs: u32, rhs: i32) -> u32 {
    match lhs as i32 + rhs {
        x => x as u32,
    }
}

impl Instruction {
    fn execute(&self, ins_ptr: u32, state: i32) -> (u32, i32) {
        match self {
            Instruction::Jump(j) => (safe_add(ins_ptr, *j), state),
            Instruction::Accumulate(v) => (ins_ptr + 1, state + v),
            Instruction::NoOp(_) => (ins_ptr + 1, state),
        }
    }
}

impl Challenge<'_> for Day8<'_> {
    fn setup(&mut self) {
        let raw_str = fs::read_to_string(self.file_path).unwrap();

        self.data = (0..)
            .zip(raw_str.lines().map(|s| Instruction::from_str(s).unwrap()))
            .collect();
    }

    fn part_1(&mut self) {
        let mut visited_instructions: HashSet<u32> = HashSet::new();
        let mut state: i32 = 0;
        let mut ins_ptr: u32 = 0;

        while !visited_instructions.contains(&ins_ptr) {
            visited_instructions.insert(ins_ptr);
            let result = self.data.get(&ins_ptr).unwrap().execute(ins_ptr, state);

            ins_ptr = result.0;
            state = result.1;
        }

        self.part_1_ans = Some(state);
    }

    fn part_2(&mut self) {
        let mut visited_instructions: HashSet<u32> = HashSet::new();
        let mut state: i32 = 0;
        let mut ins_ptr: u32 = 0;

        while self.data.get(&ins_ptr).is_some() {
            visited_instructions.insert(ins_ptr);
            let mut result = self.data.get(&ins_ptr).unwrap().execute(ins_ptr, state);

            if visited_instructions.contains(&result.0) {
                let next_ins = self.data.get(&ins_ptr).unwrap();
                let new_ins: Instruction = match next_ins {
                    Instruction::Jump(j) => Instruction::NoOp(*j),
                    Instruction::NoOp(j) => Instruction::Jump(*j),
                    Instruction::Accumulate(v) => Instruction::Accumulate(*v),
                };

                println!(
                    "Replacing instruction {:?} at address {} with {:?}",
                    self.data.get(&ins_ptr).unwrap(),
                    result.0,
                    new_ins
                );

                result = new_ins.execute(ins_ptr, state);
            }

            ins_ptr = result.0;
            state = result.1;
        }

        self.part_2_ans = Some(state);
    }

    fn format_answers(&self) -> String {
        format!(
            "Part 1: {}\nPart 2: {}",
            self.part_1_ans.unwrap_or(i32::MIN),
            self.part_2_ans.unwrap_or(i32::MIN)
        )
    }
}
