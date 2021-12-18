use std::cmp::Ordering;

use anyhow::Result;

#[derive(Debug)]
pub struct Probe {
    position: (i32, i32),
    velocity: (i32, i32),
}
impl Probe {
    pub fn new(velocity: (i32, i32)) -> Self {
        Self {
            position: (0, 0),
            velocity,
        }
    }
    pub fn step(&mut self) {
        // Increase velocity
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        // Apply drag
        self.velocity.0 += match self.velocity.0.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        // Apply gravity
        self.velocity.1 -= 1;
    }
}

#[derive(Debug)]
pub struct Area {
    x: (i32, i32),
    y: (i32, i32),
    location: AreaLocation,
}
impl Area {
    pub fn new(x: (i32, i32), y: (i32, i32)) -> Self {
        let ((i,_),(j,_)) = (x,y);
        let location = match (i, j) {
            (i, j) if i > 0 && j > 0  => AreaLocation::UpRight,
            (i, j) if i <= 0 && j > 0  => AreaLocation::UpLeft,
            (i, j) if i > 0 && j <= 0  => AreaLocation::DownRight,
            (i, j) if i <= 0 && j <= 0  => AreaLocation::DownLeft,
            other => panic!("Unexpected: {:?}", other),
        };
        Self { x, y, location}
    }
    pub fn in_area(&self, probe: &Probe) -> bool {
        probe.position.0 >= self.x.0
            && probe.position.0 <= self.x.1
            && probe.position.1 >= self.y.0
            && probe.position.1 <= self.y.1
    }
}
pub fn parse_input(input: &str) -> Area {
    let input = input.replace("target area: x=", "");
    let input = input.replace(" y=", "");
    let coords: Vec<&str> = input.split(',').collect();
    let x: Vec<&str> = coords[0].split("..").collect();
    let y: Vec<&str> = coords[1].split("..").collect();
    Area::new(
        (x[0].parse().unwrap(), x[1].parse().unwrap()),
        (y[0].parse().unwrap(), y[1].parse().unwrap()),
    )
}

#[derive(Debug)]
pub enum AreaLocation {
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

pub fn part1(input: &str) -> Result<()> {
    let area = parse_input(input);
    // Apply brute force to find all coords that fall into the area
    // Check the quadrant for the area
    let mut initial_velocity = (0, 0);
    let vel_factor = match area.location {
        AreaLocation::DownLeft => (-1, 1),
        AreaLocation::UpLeft => (-1, 1),
        AreaLocation::UpRight => (1, 1),
        AreaLocation::DownRight => (1, 1),
    };

    let mut all_ys = Vec::new();

    for i in 0..1_000_000 {
        initial_velocity.0 = vel_factor.0 * i / 1000;
        initial_velocity.1 = vel_factor.1 * i % 1000;
        let mut probe = Probe::new(initial_velocity);
        let mut max_y = i32::MIN;
        for _ in 0..1000 {
            probe.step();
            max_y = max_y.max(probe.position.1);
            if area.in_area(&probe) {
                all_ys.push(max_y);
                break;
            }
        }
    }
    println!("Max y: {}", all_ys.iter().max().unwrap());
    Ok(())
}
pub fn part2(input: &str) -> Result<()> {
    let area = parse_input(input);
    // Apply brute force to find all coords that fall into the area
    // Check the quadrant for the area
    let mut initial_velocity = (0, 0);
    let vel_factor = match area.location {
        AreaLocation::DownLeft => (-1, 1),
        AreaLocation::UpLeft => (-1, 1),
        AreaLocation::UpRight => (1, 1),
        AreaLocation::DownRight => (1, 1),
    };

    let mut velocities = Vec::new();

    for i in 0..1_000_000 {
        initial_velocity.0 = vel_factor.0 * i / 1000;
        initial_velocity.1 = vel_factor.1 * i % 1000 - 500;
        let mut probe = Probe::new(initial_velocity);
        let mut max_y = i32::MIN;
        for _ in 0..1000 {
            probe.step();
            max_y = max_y.max(probe.position.1);
            if area.in_area(&probe) {
                velocities.push(initial_velocity);
                break;
            }
        }
    }
    println!("Velocities: {}, {:?}", velocities.len(), velocities);
    Ok(())
}
