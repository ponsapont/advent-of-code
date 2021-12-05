use std::collections::HashMap;

use anyhow::Result;

pub fn read_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    let vectors: Vec<((usize, usize), (usize, usize))> = input
        .lines()
        .map(|line| {
            let vector: Vec<&str> = line.split(" -> ").collect();
            let start: Vec<&str> = vector[0].split(",").collect();
            let end: Vec<&str> = vector[1].split(",").collect();
            (
                (start[0].parse().unwrap(), start[1].parse().unwrap()),
                (end[0].parse().unwrap(), end[1].parse().unwrap()),
            )
        })
        .collect();
    // Rearrange them so they are always moving to positive
    vectors.into_iter().map(|(first,second)| {
        if first.0 > second.0 || first.1 > second.1 {
            (second,first)
        } else {
            (first, second)
        }
    }).collect()
}

/// Draws the vectors in a map
pub fn paint(vectors: Vec<&((usize, usize), (usize, usize))>) -> HashMap<(usize,usize), i32> {
    // Build a sparse matrix
    let mut sparse_matrix= HashMap::new();
    for vector in vectors {
        for i in vector.0.0..=vector.1.0 {
            for j in vector.0.1..=vector.1.1 {
                if let Some(entry) = sparse_matrix.get_mut(&(i,j)) {
                    *entry += 1;
                } else {
                    sparse_matrix.insert((i,j), 1);
                }
            }
        }
    }
    let max_x = sparse_matrix.keys().max_by(|x,y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = sparse_matrix.keys().max_by(|x,y| x.1.cmp(&y.1)).unwrap().1;
    for j in 0..=max_y {
        for i in 0..=max_x {
            if let Some(value) = sparse_matrix.get(&(i,j)) {
                print!("{}", value);
            } else {
                print!(".");
            }
        }
        println!(" ");
    }
    println!(" ");
    sparse_matrix
}

pub fn part1(input: &str) -> Result<()> {
    let vectors = read_input(input);
    // Get only the ones with maching positions
    let non_diagonal: Vec<&((usize, usize), (usize, usize))> = vectors
        .iter()
        .filter(|vector| vector.0 .0 == vector.1 .0 || vector.0 .1 == vector.1 .1)
        .collect();

    let sparse_matrix = paint(non_diagonal);
    let overlaps = sparse_matrix.values().filter(|value| **value > 1).count();

    println!("Found {} overlaps", overlaps);

    Ok(())
}

pub fn part2(_input: &str) -> Result<()> {
    Ok(())
}
