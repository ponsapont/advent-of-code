use anyhow::Result;

pub fn part1(input: &str) -> Result<()> {
    let reports: Vec<&str> = input.split('\n').collect();
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

pub fn part2(input: &str) -> Result<()> {
    let reports: Vec<&str> = input.split('\n').collect();
    let report_length = reports[0].len();

    let (mut oxigen, mut co2): (Vec<&str>, Vec<&str>) = (reports.clone(), reports);
    for idx in 0..report_length {
        oxigen = select_by_criteria(true, idx, oxigen);
        co2 = select_by_criteria(false, idx, co2);
    }
    let oxigen = i32::from_str_radix(oxigen[0], 2)?;
    let co2 = i32::from_str_radix(co2[0], 2)?;
    println!("Result: {}", (oxigen * co2));
    Ok(())
}

/// Filters by criteria for the current index.
fn select_by_criteria(is_oxigen: bool, idx: usize, reports: Vec<&str>) -> Vec<&str> {
    // Skip if only 1 report left
    if reports.len() == 1 {
        return reports;
    }
    let threshold = (reports.len() as f32 / 2.0).ceil() as usize;
    let occurences = reports
        .iter()
        .filter(|report| report.chars().nth(idx).unwrap() == '1')
        .count();
    let criteria = if occurences >= threshold {
        if is_oxigen {
            '1'
        } else {
            '0'
        }
    } else if is_oxigen {
        '0'
    } else {
        '1'
    };
    reports
        .into_iter()
        .filter(|report| report.chars().nth(idx).unwrap() == criteria)
        .collect()
}
