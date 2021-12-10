use std::{collections::HashMap};

use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut opened = Vec::new();
    let mut errors = HashMap::from([(')', 0), (']', 0), ('}', 0), ('>', 0)]);
    for line in &lines {
        for c in line {
            let last = opened.last();
            match (c, last) {
                ('(', _) | ('[', _) | ('{', _) | ('<', _) => opened.push(*c),
                (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                    opened.pop().unwrap();
                }
                (c, _) => {
                    errors.insert(*c, errors.get(c).unwrap() + 1);
                    break;
                },
            };
        }
    }
    let score = errors.iter().map(|(c, count)| match *c {
        ')' => count * 3,
        ']' => count * 57,
        '}' => count * 1197,
        '>' => count * 25137,
        _ => panic!(),
    }).sum::<i32>();

    println!("Score: {:?}", score);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut correct_lines: Vec<(&Vec<char>, Vec<char>)> = lines.iter().filter_map(|line| {
        let mut opened = Vec::new();
        for c in line {
            let last = opened.last();
            match (c, last) {
                ('(', _) | ('[', _) | ('{', _) | ('<', _) => opened.push(*c),
                (')', Some('(')) | (']', Some('[')) | ('}', Some('{')) | ('>', Some('<')) => {
                    opened.pop().unwrap();
                }
                (_, _) => {
                    return None
                },
            };
        }
        Some((line, opened))
    }).collect();

    correct_lines.iter_mut().for_each(|(_, opened)| {
        opened.reverse();
        opened.iter_mut().for_each(|c| *c = match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!(),
        });
    });
    let mut scores: Vec<i64> = correct_lines.iter().map(|(_, opened)| {
        opened.iter().fold(0, |acc, c| {
            acc * 5 + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!(),
            }
        })
    }).collect();
    scores.sort();
    println!("Result: {}", scores[scores.len()/2]);
    //correct_lines.iter().for_each(|line| println!("{:?} @ {:?}", line.0.iter().collect::<String>(), line.1.iter().collect::<String>()));
    Ok(())
}
