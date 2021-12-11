use anyhow::Result;

fn read_input(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> Result<()> {
    let numbers = read_input(input);
    let result = numbers
        .iter()
        .skip(1)
        .enumerate()
        .fold(0, |acc, (idx, num)| {
            if num > numbers.get(idx).unwrap() {
                return acc + 1;
            }
            acc
        });

    println!("Result: {}", result);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let numbers = read_input(input);
    let numbers: Vec<i32> = numbers[..numbers.len() - 2]
        .iter()
        .enumerate()
        .map(|(idx, number)| number + numbers[idx + 1] + numbers[idx + 2])
        .collect();
    // Now apply Part 1 again
    let result = numbers
        .iter()
        .skip(1)
        .enumerate()
        .fold(0, |acc, (idx, num)| {
            if num > numbers.get(idx).unwrap() {
                return acc + 1;
            }
            acc
        });
    println!("Result: {}", result);
    Ok(())
}
