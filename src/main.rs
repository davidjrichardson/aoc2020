use std::path::Path;
use std::process::exit;

use clap::{App, Arg};

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

    match day {
        1 => days::day_1::execute(input_path),
        _ => eprintln!("Day {} is not implemented", day),
    }
}
