use anyhow::Result;

pub fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split('-').collect())
        .collect()
}

pub fn find_paths<'a>(
    connections: &[Vec<&'a str>],
    curr_path: Vec<&'a str>,
    part1: bool,
) -> Vec<Vec<&'a str>> {
    // First find the next path
    let last = curr_path.last().unwrap();
    let small_visited: Vec<&&str> = curr_path[..curr_path.len() - 1]
        .iter()
        .filter(|p| {
            let mut p_lower = p.to_string();
            p_lower.make_ascii_lowercase();
            **p != "start" && p_lower == **p
        })
        .collect();
    let mut small_visited_uniq = small_visited.clone();
    small_visited_uniq.sort_unstable();
    small_visited_uniq.dedup();

    let applicables: Vec<&Vec<&str>> = connections
        .iter()
        .filter(|connection| {
            connection.contains(last)
                && if curr_path.len() > 1 {
                    !connection.contains(&"start")
                } else {
                    true
                }
                && if part1 {
                    connection
                        .iter()
                        .all(|p| !small_visited.iter().any(|q| *p == **q))
                } else {
                    // We want to find all the lowercase counts, and see that only 1 of them has
                    // more than 1 occurrence
                    if small_visited_uniq.len() != small_visited.len() {
                        connection
                            .iter()
                            .all(|p| !small_visited.iter().any(|q| *p == **q))
                    } else {
                        true
                    }
                }
        })
        .collect();

    //println!("Current path: {:?} Applicables: {:?}", curr_path, applicables);
    if applicables.is_empty() {
        return Vec::from([curr_path]);
    }
    applicables
        .iter()
        .flat_map(|applicable| {
            let next = if applicable[1] == *last {
                applicable[0]
            } else {
                applicable[1]
            };
            let mut next_path = curr_path.clone();
            next_path.push(next);
            if next != "end" {
                find_paths(connections, next_path, part1)
            } else {
                Vec::from([next_path])
            }
        })
        .collect()
}

pub fn part1(input: &str) -> Result<()> {
    let connections = parse_input(input);
    let paths = find_paths(&connections, Vec::from(["start"]), true);
    // Remove the ones not finishing at end
    let path_count = paths.iter().filter(|p| *p.last().unwrap() == "end").count();
    println!("Number of paths: {:?}", path_count);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let connections = parse_input(input);
    let paths = find_paths(&connections, Vec::from(["start"]), false);
    // Remove the ones not finishing at end
    let path_count = paths.iter().filter(|p| *p.last().unwrap() == "end").count();
    println!("Number of paths: {:?}", path_count);
    Ok(())
}
