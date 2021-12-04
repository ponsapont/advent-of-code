use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    let numbers: Vec<i32> = input
        .split("\n")
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();

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
