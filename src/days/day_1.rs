use std::time::SystemTime;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

use crate::Path;

pub fn execute(input: &Path) {
    println!("Starting day 1");

    let start_time = SystemTime::now();
    let data: Vec<i32> = fs::read_to_string(input)
        .expect("Could not read the input file")
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| i32::from_str_radix(s, 10).expect("Failed to parse integer value"))
        .sorted()
        .collect();
    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&data);
    let part_1_time = SystemTime::now();

    let part_2_ans = part_2(&data);
    let part_2_time = SystemTime::now();

    println!("\nPart 1: {}, part 2: {}", part_1_ans, part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

fn part_1(data: &Vec<i32>) -> String {
    let data_set: HashSet<i32> = data.iter().cloned().collect();
    let all_pairs: Vec<(&i32, &i32)> = data_set
        .iter()
        .cartesian_product(data_set.iter())
        .unique()
        .collect();

    let answer = all_pairs
        .iter()
        .find(|(l, r)| (*l + *r) == 2020)
        .expect("Could not find a pair of values that sums to 2020");

    format!("{}", answer.0 * answer.1)
}

fn part_2(data: &Vec<i32>) -> String {
    let data: Vec<_> = vec![data.iter(), data.iter(), data.iter()];

    let all_triples: Vec<(&i32, &i32, &i32)> = data
        .iter()
        .cloned()
        .multi_cartesian_product()
        .unique()
        .map(|v| (v[0], v[1], v[2]))
        .collect();

    let answer = all_triples
        .iter()
        .cloned()
        .find(|(l, m, r)| (*l + *m + *r) == 2020)
        .expect("Could not find a triple of values that sums to 2020");

    format!("{}", answer.0 * answer.1 * answer.2)
}
