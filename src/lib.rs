use std::{num::ParseIntError, str::FromStr};

#[macro_use]
extern crate structopt;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

/// Advent of Code 2021
#[derive(StructOpt, Debug)]
pub struct Aoc2021 {
    /// Day. If not specified, defaults to current day
    #[structopt(short, long)]
    pub day: Option<u32>,
    /// Part. If not specified, defaults to 1
    #[structopt(short, long, default_value = "1")]
    pub part: Parts,
}

#[derive(Debug)]
pub enum Parts {
    Part1,
    Part2,
}
impl FromStr for Parts {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let part = s.parse::<u32>()?;
        Ok(match part {
            1 => Self::Part1,
            2 => Self::Part2,
            other => panic!("Unsupported part: {}", other),
        })
    }
}
