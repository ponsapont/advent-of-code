use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    let reports: Vec<&str> = input.split("\n").collect();
    let report_length = reports[0].len();
    let majority_threshold = reports.len() / 2;
    let gamma: String = (0..report_length)
        .map(|idx| {
            let occurences = reports
                .iter()
                .filter(|report| report.chars().nth(idx).unwrap() == '1')
                .count();
            if occurences > majority_threshold {
                '1'
            } else {
                '0'
            }
        })
        .collect();

    let mask = u32::from_str_radix(&(0..report_length).map(|_| '1').collect::<String>(), 2)?;
    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = gamma ^ mask;

    println!("Result: {}", (gamma * epsilon));
    Ok(())
}
