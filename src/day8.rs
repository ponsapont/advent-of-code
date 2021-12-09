use std::collections::{HashMap, HashSet};

use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    // Correspondence with <num_segments, value>
    let segment_values = HashMap::from([(2, 1), (4, 4), (3, 7), (7, 8)]);

    let input: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let input: Vec<&str> = line.split(" | ").collect();
            (input[0].split(" ").collect(), input[1].split(" ").collect())
        })
        .collect();

    // Read only the values and count the known numbers
    let result = input
        .iter()
        .map(|(_, values)| values)
        .flatten()
        .filter(|value| segment_values.contains_key(&value.len()))
        .count();
    println!("Result: {}", result);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let input: Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> = input
        .lines()
        .map(|line| {
            let input: Vec<&str> = line.split(" | ").collect();
            (
                input[0]
                    .split(" ")
                    .map(|segment| segment.chars().collect())
                    .collect(),
                input[1]
                    .split(" ")
                    .map(|segment| segment.chars().collect())
                    .collect(),
            )
        })
        .collect();

    let mut numbers = Vec::new();
    for (segments, values) in &input {
        let mut segment_mappings: HashMap<char, char> = HashMap::new();
        // First find the known ones
        let mut correspondences: Vec<Option<&HashSet<char>>> = (0..10).map(|_| None).collect();
        for segment in segments {
            match segment.len() {
                2 => correspondences[1] = Some(segment),
                3 => correspondences[7] = Some(segment),
                4 => correspondences[4] = Some(segment),
                7 => correspondences[8] = Some(segment),
                _ => (),
            }
        }
        // Look for 3
        correspondences[3] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 5
                        && correspondences[7]
                            .unwrap()
                            .difference(x)
                            .collect::<Vec<&char>>()
                            .is_empty()
                })
                .unwrap(),
        );

        // Look for 6
        correspondences[6] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 6
                        && !correspondences[7]
                            .unwrap()
                            .difference(x)
                            .collect::<Vec<&char>>()
                            .is_empty()
                })
                .unwrap(),
        );

        // Look for 5
        correspondences[5] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 5
                        && correspondences[6]
                            .unwrap()
                            .difference(x)
                            .collect::<Vec<&char>>()
                            .len()
                            == 1
                })
                .unwrap(),
        );

        // Look for 2
        correspondences[2] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 5
                        && correspondences
                            .iter()
                            .find(|y| y.is_some() && *y.unwrap() == **x)
                            .is_none()
                })
                .unwrap(),
        );

        // Look for 9
        correspondences[9] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 6
                        && correspondences[4]
                            .unwrap()
                            .difference(x)
                            .collect::<Vec<&char>>()
                            .is_empty()
                })
                .unwrap(),
        );

        // Look for 0
        correspondences[0] = Some(
            segments
                .iter()
                .find(|x| {
                    x.len() == 6
                        && correspondences
                            .iter()
                            .find(|y| y.is_some() && *y.unwrap() == **x)
                            .is_none()
                })
                .unwrap(),
        );
        // println!("Correspondences: {:?}", correspondences);

        // Now find the mappings
        let correspondences: Vec<&HashSet<char>> =
            correspondences.iter().map(|x| x.unwrap()).collect();

        segment_mappings.insert(
            *correspondences[7]
                .difference(&correspondences[1])
                .collect::<Vec<&char>>()[0],
            'a',
        );
        segment_mappings.insert(
            *correspondences[5]
                .difference(&correspondences[3])
                .collect::<Vec<&char>>()[0],
            'b',
        );
        segment_mappings.insert(
            *correspondences[2]
                .difference(&correspondences[6])
                .collect::<Vec<&char>>()[0],
            'c',
        );
        segment_mappings.insert(
            *correspondences[8]
                .difference(&correspondences[0])
                .collect::<Vec<&char>>()[0],
            'd',
        );
        segment_mappings.insert(
            *correspondences[2]
                .difference(&correspondences[3])
                .collect::<Vec<&char>>()[0],
            'e',
        );
        segment_mappings.insert(
            *correspondences[3]
                .difference(&correspondences[2])
                .collect::<Vec<&char>>()[0],
            'f',
        );
        segment_mappings.insert(
            *correspondences[9]
                .difference(&correspondences[4])
                .map(|x| *x)
                .collect::<HashSet<char>>()
                .difference(correspondences[7])
                .collect::<Vec<&char>>()[0],
            'g',
        );
        // println!("Mappings: {:?}", segment_mappings);

        // Now translate the value
        let new_values: Vec<HashSet<&char>> = values
            .iter()
            .map(|value| {
                value
                    .iter()
                    .map(|c| segment_mappings.get(c).unwrap())
                    .collect()
            })
            .collect();

        // println!("Values: {:?}", values);
        // println!("New values: {:?}\n", new_values);

        let result: Vec<i32> = new_values
            .iter()
            .map(|val| {
                let mut as_vec = val.iter().map(|c| **c).collect::<Vec<char>>();
                as_vec.sort();
                let string = as_vec.iter().collect::<String>();
                match string.as_ref() {
                    "abcefg" => 0,
                    "cf" => 1,
                    "acdeg" => 2,
                    "acdfg" => 3,
                    "bcdf" => 4,
                    "abdfg" => 5,
                    "abdefg" => 6,
                    "acf" => 7,
                    "abcdefg" => 8,
                    "abcdfg" => 9,
                    other => panic!("Unsupported number: {:?}", other),
                }
            })
            .collect();

        let number = result
            .iter()
            .map(|n| n.to_string().chars().next().unwrap())
            .collect::<String>();
        numbers.push(number.parse::<i32>().unwrap());
    }
    println!("Result: {}", numbers.iter().sum::<i32>());
    Ok(())
}
