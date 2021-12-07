use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    let positions: Vec<i32> = input.split(",").map(|line| line.parse().unwrap()).collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let res: (i32, i32) = (min..=max)
        .map(|i| {
            (
                i,
                positions.iter().map(|position| (position - i).abs()).sum(),
            )
        })
        .min_by(|x: &(i32, i32), y| x.1.cmp(&y.1))
        .unwrap();

    println!("Result: {:?}", res);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let positions: Vec<i32> = input.split(",").map(|line| line.parse().unwrap()).collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let res: (i32, i32) = (min..=max)
        .map(|i| {
            (
                i,
                positions.iter().map(|position|calc_fuel(*position, i)).sum(),
            )
        })
        .min_by(|x: &(i32, i32), y| x.1.cmp(&y.1))
        .unwrap();

    println!("Result: {:?}", res);
    Ok(())
}

pub fn calc_fuel(prev_position: i32, new_position: i32) -> i32 {
    let num_steps = (prev_position - new_position).abs();

    let mut fuel = 0;
    for i in 1..=num_steps {
        fuel += i
    }
    fuel
}