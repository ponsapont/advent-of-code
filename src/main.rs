use chrono::{Datelike, Utc};

extern crate advent_of_code_2021;

use advent_of_code_2021::*;
use anyhow::Result;
use structopt::StructOpt;

pub fn main() -> Result<()> {
    let args = Aoc2021::from_args();
    let day = if let Some(day) = args.day {
        day
    } else {
        Utc::today().day()
    };

    let input = std::fs::read_to_string(format!("input/day{}.txt", day))
        .expect(&format!("Input for day {} not found!", day));
    match day {
        1 => match args.part {
            Parts::Part1 => day1::part1(&input)?,
            Parts::Part2 => day1::part2(&input)?,
        },
        2 => match args.part {
            Parts::Part1 => day2::part1(&input)?,
            Parts::Part2 => day2::part2(&input)?,
        },
        3 => match args.part {
            Parts::Part1 => day3::part1(&input)?,
            Parts::Part2 => day3::part2(&input)?,
        },
        4 => match args.part {
            Parts::Part1 => day4::part1(&input)?,
            Parts::Part2 => day4::part2(&input)?,
        },
        day => panic!(format!("Day {} not implemented!", day)),
    };
    Ok(())
}
