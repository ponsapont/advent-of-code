use anyhow::Result;

type Instruction = (usize, usize);
type Fold = (char, usize);

pub fn parse_input(input: &str) -> (Vec<Instruction>, Vec<Fold>) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let input = (
        input[0]
            .lines()
            .map(|line| {
                let line: Vec<&str> = line.split(',').collect();
                (line[0].parse().unwrap(), line[1].parse().unwrap())
            })
            .collect(),
        input[1]
            .lines()
            .map(|line| {
                let line: Vec<&str> = line.split(' ').last().unwrap().split('=').collect();
                (line[0].chars().next().unwrap(), line[1].parse().unwrap())
            })
            .collect(),
    );
    input
}

pub fn print(coords: &[Instruction]) {
    let width = coords.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let height = coords.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    for y in 0..=height {
        for x in 0..=width {
            if coords.iter().any(|(i, j)| *i == x && *j == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn do_fold(coords: &mut Vec<Instruction>, fold: &Fold) {
    match fold {
        ('x', x) => {
            coords.iter_mut().filter(|(i, _)| i > x).for_each(|(i, _)| {
                *i = *x - (*i - *x);
            });
        }
        ('y', y) => {
            coords.iter_mut().filter(|(_, j)| j > y).for_each(|(_, j)| {
                *j = *y - (*j - *y);
            });
        }
        other => panic!("Not supported: {:?}", other),
    }
    coords.sort_unstable();
    coords.dedup();
    //print(coords);
}

pub fn part1(input: &str) -> Result<()> {
    let (mut coords, folds) = parse_input(input);
    //print(&coords);
    do_fold(&mut coords, &folds[0]);
    println!("Result: {}", coords.len());
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let (mut coords, folds) = parse_input(input);
    for fold in folds {
        do_fold(&mut coords, &fold);
    }
    print(&coords);
    println!("Result: {}", coords.len());
    Ok(())
}

// ####..##..####.#..#.#....#..#.####.####
// #....#..#.#....#..#.#....#..#....#.#...
// ###..#....###..####.#....####...#..###.
// #....#....#....#..#.#....#..#..#...#...
// #....#..#.#....#..#.#....#..#.#....#...
// ####..##..#....#..#.####.#..#.####.#...

// ECFHLHZF
