use std::time::SystemTime;
use itertools::Itertools;
use std::fs;

use crate::Path;

pub fn execute(input: &Path) {
    println!("Starting day 1");

    let start_time = SystemTime::now();
    let data_raw: Vec<i32> = fs::read_to_string(input)
        .expect("Could not read the input file")
        .lines()
        .map(|s| i32::from_str_radix(s, 10).expect("Failed to parse integer value"))
        .sorted()
        .collect();

    let data: Vec<i32> = data_raw
        .iter()
        .cloned()
        .filter(|x| x + data_raw.get(0).unwrap() <= 2020)
        .collect();

    let setup_time = SystemTime::now();

    let part_1_ans = get_combinations(&data, 2);
    let part_1_time = SystemTime::now();

    let part_2_ans = get_combinations(&data, 3);
    let part_2_time = SystemTime::now();

    println!("Part 1 numbers: {:?}, answer: {:?}", part_1_ans, part_1_ans.iter().cloned().fold1(|x, y| x*y).unwrap());
    println!("Part 2 numbers: {:?}, answer: {:?}", part_2_ans, part_2_ans.iter().cloned().fold1(|x, y| x*y).unwrap());

    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

fn get_combinations(nums: &Vec<i32>, size: usize) -> Vec<i32> {
    nums.iter()
        .cloned()
        .combinations(size)
        .find(|v| v.iter().sum::<i32>() == 2020)
        .unwrap()
}
