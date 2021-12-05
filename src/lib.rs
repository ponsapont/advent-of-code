use std::{num::ParseIntError, str::FromStr};

#[macro_use]
extern crate structopt;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

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
            other => panic!(format!("Unsupported part: {}", other)),
        })
    }
}
