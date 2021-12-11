use anyhow::Result;


pub fn part1(input: &str) -> Result<()> {
    let mut state: Vec<i32> = input.split(',').map(|line| line.parse().unwrap()).collect();
    //println!("Initial state: {:?}", state);

    let num_days = 80;

    for _i in 1..=num_days {
        // Update state
        state.iter_mut().for_each(|counter| *counter -=1);
        // Count how many items to add
        let num_new = state.iter_mut().filter(|counter| **counter < 0).count();
        // Update negatives to 6
        state.iter_mut().filter(|counter| **counter < 0).for_each(|counter| *counter = 6);
        // Append 8
        state.extend((0..num_new).map(|_| 8));
        //println!("After {} days: {:?}", i, state);
    }
    println!("Result: {}", state.len());
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let state: Vec<i64> = input.split(',').map(|line| line.parse().unwrap()).collect();

    // Crate a vector with <timer>,<num_occurrences>
    let mut state: Vec<(i64,i64)> = state.iter().map(|timer| (*timer,1)).collect();

    // Join the entries with the same timer
    state = join_occurences(state);
    println!("Initial state: {:?}", state);

    let num_days = 256;

    for _i in 1..=num_days {
        // Update state
        state.iter_mut().for_each(|(counter,_)| *counter -=1);
        // Count how many items to add
        let num_new = if let Some(found) = state.iter_mut().find(|(counter, _)| *counter < 0) {
            found.1
        } else {
            0
        };
        // Update negatives to 6
        state.iter_mut().filter(|(counter,_)| *counter < 0).for_each(|(counter,_)| *counter = 6);
        // Append 8 (at this point we never had 8)
        state.push((8, num_new));
        state = join_occurences(state);
       //println!("After {} days: {:?}", i, state);
    }
    println!("Result: {}", state.iter().fold(0, |acc, (_, occurences)| acc + occurences));
    Ok(())
}

pub fn join_occurences(state: Vec<(i64,i64)>) -> Vec<(i64,i64)>{
    // Join the entries with the same timer
    let mut new_state = Vec::new();
    for i in 0..=8 {
        let occurrences = state.iter().filter_map(|(timer, occurences)| if *timer == i {
            Some(occurences)
        } else {
            None
        }).sum::<i64>();
        if occurrences > 0 {
            new_state.push((i, occurrences));
        }
    }
    new_state
}
