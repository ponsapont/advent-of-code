use std::collections::HashMap;

use anyhow::Result;

type Rule = (String, char);

pub fn parse_input(input: &str) -> (String, Vec<Rule>) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let input = (
        input[0].to_owned(),
        input[1]
            .lines()
            .map(|line| {
                let line: Vec<&str> = line.split(" -> ").collect();
                (line[0].to_owned(), line[1].chars().next().unwrap())
            })
            .collect(),
    );
    input
}

pub fn part1(input: &str) -> Result<()> {
    let (polymer, rules) = parse_input(input);

    let num_steps = 40;
    // Iterate in reverse order, so the indexes don't change
    let mut next_polymer = polymer;
    for step in 1..=num_steps {
        let polymer = next_polymer;
        next_polymer = polymer.clone();
        println!("Step {}. Polymer length: {}", step, next_polymer.len());
        for i in (1..polymer.len()).rev() {
            let pattern = format!(
                "{}{}",
                polymer.chars().nth(i - 1).unwrap(),
                polymer.chars().nth(i).unwrap()
            );
            if let Some((_, c)) = rules.iter().find(|(rule, _)| pattern == *rule) {
                next_polymer.insert(i, *c);
            } else {
                // If the pattern is not found, we can just remove the indexes, as they will never be considered again
                println!("Remove");
                next_polymer.remove(i);
                next_polymer.remove(i - 1);
            }
        }
    }

    let chars: Vec<char> = next_polymer.clone().chars().collect();
    let mut unique_chars = chars.clone();
    unique_chars.sort_unstable();
    unique_chars.dedup();
    let char_count: Vec<(char, usize)> = unique_chars
        .iter()
        .map(|c| (*c, chars.iter().filter(|d| **d == *c).count()))
        .collect();

    let max = char_count.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let min = char_count.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    println!("Result: {}", (max - min));
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    // For part 2, we use a recursive tree approach.
    // We start with the input string pairs. Each recursion is a step
    // We update the counters at each recursion step
    let (polymer, rules) = parse_input(input);
    let num_steps = 40;
    // We keep track of the occurrences of each active pair
    let initial_state: Vec<((char, char), i64)> = polymer
        .chars()
        .zip(polymer[1..].chars())
        .map(|s| (s, 1))
        .collect();
    let mut counters = HashMap::new();
    polymer.chars().for_each(|c| {
        let val = counters.entry(c).or_insert(0);
        *val += 1;
    });

    let mut state = initial_state;
    for _step in 1..=num_steps {
        let next_state: Vec<((char, char), i64)> = state
            .iter()
            .flat_map(|(pair, count)| {
                let rule = rules
                    .iter()
                    .find(|r| r.0 == format!("{}{}", pair.0, pair.1))
                    .unwrap();
                let val = counters.entry(rule.1).or_insert(0);
                *val += *count;
                [((pair.0, rule.1), *count), ((rule.1, pair.1), *count)]
            })
            .collect();
        // Create an unique list with updated counters
        let mut unique: Vec<(char, char)> =
            next_state.iter().map(|(pair, _)| pair).cloned().collect();
        unique.sort_unstable();
        unique.dedup();
        let next_state = unique
            .iter()
            .map(|pair| {
                (
                    *pair,
                    next_state
                        .iter()
                        .filter(|x| x.0 == *pair)
                        .map(|x| x.1)
                        .sum(),
                )
            })
            .collect();
        state = next_state;
    }
    println!("Counters: {:?}", counters);
    let max = counters.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    let min = counters.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().1;
    println!("Result: {}", (max - min));
    Ok(())
}
