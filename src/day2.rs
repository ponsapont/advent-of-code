use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    units: i32,
}
#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}
impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            other => panic!("Invalid direction: {}", other),
        })
    }
}

fn read_input(input: &str) -> Vec<Command> {
    input
        .split('\n')
        .into_iter()
        .map(|line| {
            let command: Vec<&str> = line.split_whitespace().collect();
            let direction = command[0].parse().unwrap();
            let units = command[1].parse().unwrap();
            Command { direction, units }
        })
        .collect()
}

pub fn part1(input: &str) -> Result<()> {
    let commands = read_input(input);

    let mut horizontal = 0;
    let mut vertical = 0;

    for command in &commands {
        match command.direction {
            Direction::Down => vertical += command.units,
            Direction::Up => vertical -= command.units,
            Direction::Forward => horizontal += command.units,
        }
    }
    println!("Result: {}", (horizontal * vertical));
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let commands = read_input(input);

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut depth = 0;

    for command in &commands {
        match command.direction {
            Direction::Down => vertical += command.units,
            Direction::Up => vertical -= command.units,
            Direction::Forward => {
                horizontal += command.units;
                depth += vertical * command.units;
            }
        }
    }
    println!("Result: {}", (horizontal * depth));
    Ok(())
}
