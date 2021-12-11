use anyhow::Result;

use std::collections::HashSet;

pub fn part1(input: &str) -> Result<()> {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let map_width = map[0].len();
    let map_height = map.len();
    // Find the low points
    let mut low_points = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            // Check the four directions
            if  !(x > 0 && map[y][x - 1] <= *point ||
                y > 0 && map[y - 1][x] <= *point ||
                x + 1 < map_width && map[y][x + 1] <= *point ||
                y + 1 < map_height && map[y + 1][x] <= *point) {
                    low_points.push(*point);
            }
        }
    }
    println!("Result: {}", low_points.iter().map(|x| *x+1).sum::<i32>());
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let map_width = map[0].len();
    let map_height = map.len();
    // Find the low points
    let mut low_points = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            // Check the four directions
            if  !(x > 0 && map[y][x - 1] <= *point ||
                y > 0 && map[y - 1][x] <= *point ||
                x + 1 < map_width && map[y][x + 1] <= *point ||
                y + 1 < map_height && map[y + 1][x] <= *point) {
                    low_points.push((y,x,*point));
            }
        }
    }
    println!("Result: {}", low_points.iter().map(|x| x.2+1).sum::<i32>());

    // Now iteratively check the basins until the result doesn't change, for each low point
    let mut basins = Vec::new();
    for (y, x, point) in &low_points {
        if *point >= 8 {
            continue
        }
        let mut basin = HashSet::from([(*y, *x, *point)]);

        for next_point in *point..=7 {
            let next_basin: Vec<(usize, usize, i32)> = basin.iter().cloned().filter(|x| x.2 <= next_point).collect();
            for (y, x, _) in next_basin {
                let coords: [(i32,i32); 4] = [
                    (y as i32, x as i32 - 1),
                    (y as i32 - 1, x as i32),
                    (y as i32, x as i32 + 1),
                    (y as i32 + 1, x as i32)
                ];

                for (i,j) in coords {
                    if i < 0 || j < 0 || i >= map_height as i32 || j >= map_width as i32{
                        continue;
                    }
                    let (i, j) = (i as usize, j as usize);
                    if map[i][j] == next_point+1 {
                        let value = map[i][j];
                        basin.insert((i, j, value));
                    }
                }
            }
        }
        basins.push(basin);
    }
    let mut lengths = basins.iter().map(|x| x.len()).collect::<Vec<usize>>();
    lengths.sort_unstable();
    let lengths = &lengths[lengths.len()-3..];
    println!("Basins: {:?}", lengths.iter().product::<usize>());
    Ok(())
}