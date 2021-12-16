use std::{cell::RefCell, cmp::Ordering, collections::BinaryHeap, fmt::Debug, rc::Rc};

use anyhow::Result;

#[derive(PartialEq, Eq)]
pub struct Node {
    pub distance: i32,
    pub weight: i32,
    pub adjacents: Vec<Rc<RefCell<Node>>>,
    pub visited: bool,
}

impl Node {
    pub fn new(weight: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            distance: i32::MAX,
            adjacents: Vec::new(),
            visited: false,
            weight,
        }))
    }
    pub fn add_adjacent(&mut self, node: Rc<RefCell<Node>>) {
        self.adjacents.push(node);
    }
    pub fn increase_distance(&mut self, distance: i32) {
        self.distance += distance;
    }
    pub fn set_visited(&mut self) {
        self.visited = true;
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("distance", &self.distance)
            .field("weight", &self.weight)
            .field("visited", &self.visited)
            .field(
                "adjacents",
                &self
                    .adjacents
                    .iter()
                    .map(|i| i.borrow().weight)
                    .collect::<Vec<i32>>(),
            )
            .finish()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.distance.partial_cmp(&other.distance) {
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => Some(Ordering::Equal),
            other => other,
        }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.distance.cmp(&other.distance) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Rc<RefCell<Node>>> {
    let graph: Vec<Rc<RefCell<Node>>> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|x| Node::new(x.to_string().parse().unwrap()))
                .collect::<Vec<Rc<RefCell<Node>>>>()
        })
        .collect();

    // Transform the grid into a graph
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    graph.iter().enumerate().for_each(|(idx, node)| {
        let (i, j) = (idx / width, idx % width);
        let coords = [
            (i as i32 - 1, j as i32),
            (i as i32, j as i32 - 1),
            (i as i32 + 1, j as i32),
            (i as i32, j as i32 + 1),
        ];
        //print!("Node: {},{} ", i,j);
        for (x, y) in coords {
            if x >= 0 && y >= 0 && x < height as i32 && y < width as i32 {
                //print!("- {},{} ", x,y);
                node.borrow_mut()
                    .add_adjacent(graph[x as usize * height as usize + y as usize].clone());
            }
        }
    });
    graph
}

pub fn dijkstra(graph: &[Rc<RefCell<Node>>], last: Rc<RefCell<Node>>) {
    // Implement Dijkstra.
    // Start from all unvisited
    // Set distance 0 to the first node
    graph.first().unwrap().borrow_mut().distance = 0;
    let mut unvisited: BinaryHeap<Rc<RefCell<Node>>> = BinaryHeap::new();

    unvisited.push(graph.first().unwrap().clone());
    // Update the distances
    while !unvisited.is_empty() {
        let node = unvisited.pop().unwrap();
        // let node = unvisited.first().unwrap();
        for adjacent in node
            .borrow()
            .adjacents
            .iter()
            .filter(|adjacent| !adjacent.borrow().visited)
        {
            {
                let mut adjacent = adjacent.borrow_mut();
                adjacent.distance = adjacent
                    .distance
                    .min(node.borrow().distance + adjacent.weight);
            }
            if !adjacent.borrow().visited
                && !unvisited.iter().any(|n| n.as_ptr() == adjacent.as_ptr())
            {
                unvisited.push(adjacent.clone());
            }
        }
        node.borrow_mut().set_visited();
        if node.as_ptr() == last.as_ptr() {
            return;
        }
    }
}
pub fn part1(input: &str) -> Result<()> {
    let unvisited = parse_input(input);
    let last = unvisited.last().unwrap().clone();

    dijkstra(&unvisited, last.clone());

    println!("Last: {:?}", last);
    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    // First prepare the input
    let times_bigger = 5;

    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut grid = Vec::new();

    let width = input.get(0).unwrap().len();
    let height = input.len();

    for i in 0..(width * times_bigger) {
        grid.push(String::new());
        for j in 0..(height * times_bigger) {
            let value =
                (input[i % width][j % height] as usize + i / width + j / height - 1) % 9 + 1;
            let value = value.to_string();
            grid.last_mut().unwrap().push_str(&value);
        }
    }
    let grid = grid[1..]
        .iter()
        .fold(grid[0].to_string(), |acc, l| format!("{}\n{}", acc, l));

    let unvisited = parse_input(&grid);
    let last = unvisited.last().unwrap().clone();

    dijkstra(&unvisited, last.clone());

    println!("Last: {:?}", last);
    Ok(())
}
