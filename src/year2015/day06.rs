use regex::Regex;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

pub struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<i32>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, fill: i32) -> Self {
        Self {rows, cols, data: vec![fill; rows * cols]}
    }

    pub fn get(&self, Position{row, col}: &Position) -> &i32 {
        &self.data[row * self.cols + col]
    }
    
    pub fn get_mut(&mut self, Position{row, col}: &Position) -> &mut i32 {
        &mut self.data[row * self.cols + col]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Command {
    start: Position,
    end: Position,
    act: Action,
}

pub fn parse(input: &str) -> Vec<Command> {
    let re = Regex::new("[0-9]+").unwrap();
    input.split('\n').filter(|s| !s.is_empty()).map(|cmd| {
        let act: Action = match cmd.chars().nth(6) {
            Some('n') => Action::TurnOn,
            Some('f') => Action::TurnOff,
            Some(' ') => Action::Toggle,
            x => unreachable!("{cmd:?}: {x:?}"),
        };

        let inds: Vec<_> = re.find_iter(cmd).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
        Command {start: Position{col: inds[0], row: inds[1]}, end: Position{col: inds[2], row: inds[3]}, act}
    }).collect() 
}

pub fn part1(input: &[Command]) -> i32 {
    let mut grid = Grid::new(1000, 1000, 0);
    for cmd in input {
        for row in cmd.start.row..=cmd.end.row {
            for col in cmd.start.col..=cmd.end.col {
                let p = Position{row, col};
                match cmd.act {
                    Action::TurnOn => *grid.get_mut(&p) = 1,
                    Action::TurnOff => *grid.get_mut(&p) = 0,
                    Action::Toggle => *grid.get_mut(&p) = 1 - grid.get(&p),
                }
            }
        }
    }

    let mut total: i32 = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            total += grid.get(&Position{row, col});
        }
    }

    total
}

pub fn part2(input: &[Command]) -> i32 {
    let mut grid = Grid::new(1000, 1000, 0);
    for cmd in input {
        for row in cmd.start.row..=cmd.end.row {
            for col in cmd.start.col..=cmd.end.col {
                let p = Position{row, col};
                match cmd.act {
                    Action::TurnOn => *grid.get_mut(&p) = grid.get(&p) + 1,
                    Action::TurnOff => *grid.get_mut(&p) = max(0, grid.get(&p) - 1),
                    Action::Toggle => *grid.get_mut(&p) = grid.get(&p) + 2,
                }
            }
        }
    }

    let mut total: i32 = 0;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            total += grid.get(&Position{row, col});
        }
    }

    total
}
