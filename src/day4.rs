use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Board {
    content: Vec<Vec<i32>>,
    marked: Vec<(usize, usize)>,
    board_width: usize,
    winning_number: Option<i32>,
}
impl Board {
    pub fn new(content: Vec<Vec<i32>>) -> Self {
        Self {
            board_width: content[0].len(),
            content,
            marked: Vec::new(),
            winning_number: None,
        }
    }
    /// Search for the given number in `content` and registers it in the `marked` list if found.
    pub fn mark(&mut self, number: &i32) {
        let found = self.content.iter().enumerate().find_map(|(row_idx, row)| {
            let col_idx = row.iter().enumerate().find_map(|(col_idx, cell)| {
                if *cell == *number{
                    Some(col_idx)
                } else {
                    None
                }
            });
            if let Some(col_idx) = col_idx {
                Some((row_idx, col_idx))
            } else {
                None
            }
        });
        if let Some(found) = found {
            self.marked.push(found);
        }
    }

    /// Iterates over the marked cells and determines if the current board is a winner
    pub fn is_winner(&mut self, number: &i32) -> bool {
        self.mark(number);
        if self.marked.len() < self.board_width {
            return false;
        }
        for idx in 0..self.board_width {
            let lines = self.marked.iter().filter(|coord| coord.0 == idx).count();
            if lines == self.board_width {
                self.winning_number = Some(*number);
                return true;
            }
            let columns = self.marked.iter().filter(|coord| coord.1 == idx).count();
            if columns == self.board_width {
                self.winning_number = Some(*number);
                return true;
            }
        }
        false
    }

    // Get the score
    pub fn get_score(&self) -> i32 {
        if self.winning_number.is_none() {
            return 0;
        }
        let winning_number = self.winning_number.unwrap();
        let score = self.content.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, col| acc + col)
        });
        // Remove now the marked ones
        let score = self.marked.iter().fold(score, |acc, (x,y)| {
            acc - self.content[*x][*y]
        });
        score * winning_number
    }
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let numbers = chunks[0].split(",").map(|x| x.parse().unwrap()).collect();
    let boards = chunks[1..]
        .iter()
        .map(|chunk| {
            let content = chunk
                .split("\n")
                .map(|line| {
                    line.split(" ")
                        .filter(|x| !x.is_empty())
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();
            Board::new(content)
        })
        .collect();
    (numbers, boards)
}


pub fn part1(input: &str) -> Result<()> {
    let (numbers, mut boards) = parse_input(input);

    let mut winner = None;
    for number in &numbers {
        for board in &mut boards {
            if board.is_winner(number) {
                winner = Some(board.clone());
                break
            }
        }
        if winner.is_some() {
            break;
        }
    }

    if let Some(winner) = winner {
        println!("Winner found with number {} and score {}", winner.winning_number.unwrap(), winner.get_score());
    }
    Ok(())
}

pub fn part2(_input: &str) -> Result<()> {
    Ok(())
}
