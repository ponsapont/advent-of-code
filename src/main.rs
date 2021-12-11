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
        5 => match args.part {
            Parts::Part1 => day5::part1(&input)?,
            Parts::Part2 => day5::part2(&input)?,
        },
        6 => match args.part {
            Parts::Part1 => day6::part1(&input)?,
            Parts::Part2 => day6::part2(&input)?,
        },
        7 => match args.part {
            Parts::Part1 => day7::part1(&input)?,
            Parts::Part2 => day7::part2(&input)?,
        },
        8 => match args.part {
            Parts::Part1 => day8::part1(&input)?,
            Parts::Part2 => day8::part2(&input)?,
        },
        9 => match args.part {
            Parts::Part1 => day9::part1(&input)?,
            Parts::Part2 => day9::part2(&input)?,
        },
        10 => match args.part {
            Parts::Part1 => day10::part1(&input)?,
            Parts::Part2 => day10::part2(&input)?,
        },
        11 => match args.part {
            Parts::Part1 => day11::part1(&input)?,
            Parts::Part2 => day11::part2(&input)?,
        },
        day => panic!("Day {} not implemented!", day),
    };
    Ok(())
}
