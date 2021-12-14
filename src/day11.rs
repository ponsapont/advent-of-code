use std::collections::HashSet;

use anyhow::Result;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn perform_step(_step: i32, octopuses: &mut Vec<Vec<i32>>) -> (usize, usize) {
    let mut num_flashes = 0;
    let mut simultaneous = 0;
    let height = octopuses.len() as i32;
    let width = octopuses[0].len() as i32;
    // println!("Step: {}", step);
    // Step 1: Increase numbers and retrieve the next to flash
    let mut to_flash: HashSet<(usize, usize)> = octopuses
        .iter_mut()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter_mut()
                .enumerate()
                .filter_map(|(j, o)| {
                    *o += 1;
                    if *o > 9 {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect();

    // Step 2: Perform the flash.
    // println!("To flash: {:?}", to_flash);
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    while !to_flash.is_empty() {
        flashed.extend(to_flash.iter());
        to_flash = to_flash
            .iter()
            .flat_map(|(i, j)| {
                (-1..=1)
                    .flat_map(|k| {
                        (-1..=1)
                            .filter_map(|l| {
                                let x = *i as i32 + k;
                                let y = *j as i32 + l;
                                if x >= 0 && y >= 0 && x < height && y < width {
                                    octopuses[x as usize][y as usize] += 1;
                                    if octopuses[x as usize][y as usize] > 9 {
                                        return Some((x as usize, y as usize));
                                    }
                                }
                                None
                            })
                            .collect::<HashSet<(usize, usize)>>()
                    })
                    .collect::<HashSet<(usize, usize)>>()
            })
            .collect::<HashSet<(usize, usize)>>();
        to_flash = to_flash.difference(&flashed).cloned().collect();
        // println!("To flash: {:?}", to_flash);
    }
    if flashed.len() == width as usize * height as usize {
        simultaneous += 1;
    }
    num_flashes += flashed.len();
    // Set to 0 the flashed ones
    octopuses.iter_mut().for_each(|l| {
        l.iter_mut().for_each(|o| {
            if *o > 9 {
                *o = 0
            }
        })
    });
    (num_flashes, simultaneous)
    // octopuses.iter().for_each(|o| println!("{:?}", o));
    // println!();
}

pub fn part1(input: &str) -> Result<()> {
    let mut octopuses = parse_input(input);
    let num_steps = 100;

    let mut num_flashes = 0;
    for step in 1..=num_steps {
        num_flashes += perform_step(step, &mut octopuses).0;
    }
    println!("Result: {}", num_flashes);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let mut octopuses = parse_input(input);

    let mut step = 0;
    loop {
        step += 1;
        if perform_step(step, &mut octopuses).1 > 0 {
            break;
        };
    }
    println!("Result: {}", step);
    Ok(())
}
