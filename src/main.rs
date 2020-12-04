#[macro_use] extern crate lazy_static;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

use clap::{App, Arg};
use days::challenge_from_day;

mod days;

fn main() {
    let matches = App::new("Advent of Code 2020")
        .version("0.1.0")
        .author("David Richardson <david@tankski.co.uk>")
        .about("A simple CLI for building and running my AoC 2020 solutions")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_name("DAY")
                .about("The day of the challenge to run")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .about("The challenge input file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let day: u32 = matches.value_of_t_or_exit("day");
    let input_path = match matches
        .value_of("input")
        .and_then(|path_str| Some(Path::new(path_str)))
    {
        Some(p) => p,
        None => Path::new(""),
    };

    if input_path == Path::new("") {
        eprintln!("Please provide an input file for the challenge");
        exit(1);
    }

    if !input_path.exists() {
        eprintln!("Input file \"{:?}\" does not exist", input_path);
    }

    let exit_code: i32 = challenge_from_day(&day, input_path).map_or_else(
        |err| {
            eprintln!("Failed to run day {}: {}", &day, err);
            -1
        },
        |mut c| {
            println!("Starting day {}\n", &day);

            let start_time = SystemTime::now();
            c.setup();
            let setup_time = SystemTime::now();

            c.part_1();
            let part_1_time = SystemTime::now();

            c.part_2();
            let part_2_time = SystemTime::now();

            println!("Answers for day {}:", day);
            println!("{}", c.format_answers());

            println!(
                "\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
                setup_time.duration_since(start_time).unwrap(),
                part_1_time.duration_since(setup_time).unwrap(),
                part_2_time.duration_since(part_1_time).unwrap(),
                part_2_time.duration_since(start_time).unwrap()
            );

            0
        },
    );

    exit(exit_code);
}
