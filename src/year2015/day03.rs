use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self{x, y}
    }

    pub fn step(&mut self, dir: Directions) {
        match dir {
            Directions::Up => self.y += 1,
            Directions::Down => self.y -= 1,
            Directions::Left => self.x -= 1,
            Directions::Right => self.x += 1,
        };
    }
}

pub fn parse(input: &str) -> Vec<Directions> {
    let mut data = Vec::<Directions>::new();
    for c in input.chars() {
        match c {
            '^' => data.push(Directions::Up),
            'v' => data.push(Directions::Down),
            '<' => data.push(Directions::Left),
            '>' => data.push(Directions::Right),
            _ => (),
        }
    }

    data
}

pub fn part1(input: &[Directions]) -> usize {
    let mut pos = Position::new(0, 0);
    let mut h: HashSet<Position> = HashSet::new();
    h.insert(pos);

    for d in input {
        pos.step(*d);
        h.insert(pos);
    }

    h.len()
}

pub fn part2(input: &[Directions]) -> usize {
    let mut p1 = Position::new(0, 0);
    let mut p2 = Position::new(0, 0);
    let mut h: HashSet<Position> = HashSet::new();
    h.insert(p1);

    for (i, d) in input.iter().enumerate() {
        if i % 2 == 0 {
            p1.step(*d);
            h.insert(p1);
        } else {
            p2.step(*d);
            h.insert(p2);
        }
    }

    h.len()
}
