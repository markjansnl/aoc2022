#![feature(hash_drain_filter)]

use std::collections::HashMap;

pub mod input;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Default)]
pub struct Grove(HashMap<Position, Vec<Position>>);

pub struct Neighbours {
    nw: bool,
    n: bool,
    ne: bool,
    w: bool,
    e: bool,
    sw: bool,
    s: bool,
    se: bool,
}

impl From<&str> for Grove {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| {
                        (c == '#').then_some((
                            Position {
                                x: x as isize,
                                y: y as isize,
                            },
                            Vec::new(),
                        ))
                    })
                })
                .flatten()
                .collect(),
        )
    }
}

impl Grove {
    pub fn round(&mut self, start_rule: u8) {
        let mut grove: HashMap<Position, Vec<Position>> = HashMap::new();
        for position in self.0.keys().copied() {
            let next_position = self.next_position(position, start_rule);
            grove
                .entry(next_position)
                .and_modify(|vec| vec.push(position))
                .or_insert(vec![position]);
        }

        let reset_positions = grove
            .drain_filter(|_, vec| vec.len() > 1)
            .map(|(_, vec)| vec.into_iter())
            .flatten()
            .collect::<Vec<_>>();
        for position in reset_positions {
            grove.insert(position, Vec::new());
        }

        self.0 = grove;
    }

    fn next_position(&self, position: Position, start_rule: u8) -> Position {
        let neighbours = self.neighbours(position);
        if neighbours.is_empty() {
            return position;
        }

        neighbours
            .propose_next_position(position, start_rule)
            .or_else(|| {
                neighbours
                    .propose_next_position(position, (start_rule + 1) % 4)
                    .or_else(|| {
                        neighbours
                            .propose_next_position(position, (start_rule + 2) % 4)
                            .or_else(|| {
                                neighbours.propose_next_position(position, (start_rule + 3) % 4)
                            })
                    })
            })
            .unwrap_or(position)
    }

    fn neighbours(&self, position: Position) -> Neighbours {
        let Position { x, y } = position;
        Neighbours {
            nw: self.0.contains_key(&Position { x: x - 1, y: y - 1 }),
            n: self.0.contains_key(&Position { x, y: y - 1 }),
            ne: self.0.contains_key(&Position { x: x + 1, y: y - 1 }),
            w: self.0.contains_key(&Position { x: x - 1, y }),
            e: self.0.contains_key(&Position { x: x + 1, y }),
            sw: self.0.contains_key(&Position { x: x - 1, y: y + 1 }),
            s: self.0.contains_key(&Position { x, y: y + 1 }),
            se: self.0.contains_key(&Position { x: x + 1, y: y + 1 }),
        }
    }

    pub fn count_empty_ground(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.0.keys().fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN), |(min_x, max_x, min_y, max_y), position| {
            (min_x.min(position.x), max_x.max(position.x), min_y.min(position.y), max_y.max(position.y))
        });
        (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - self.0.len()
    }
}

impl Neighbours {
    pub fn is_empty(&self) -> bool {
        [
            self.nw, self.n, self.ne, self.w, self.e, self.sw, self.s, self.se,
        ]
        .into_iter()
        .all(|neighbour| !neighbour)
    }

    pub fn propose_next_position(&self, position: Position, rule: u8) -> Option<Position> {
        let Position { x, y } = position;
        match rule {
            0 => (!self.n && !self.ne && !self.nw).then_some(Position { x, y: y - 1 }),
            1 => (!self.s && !self.se && !self.sw).then_some(Position { x, y: y + 1 }),
            2 => (!self.w && !self.nw && !self.sw).then_some(Position { x: x - 1, y }),
            3 => (!self.e && !self.ne && !self.se).then_some(Position { x: x + 1, y }),
            _ => unreachable!("Wrong rule number!"),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut grove = Grove::from(input);
    for round in 0..10 {
        grove.round(round % 4);
    }
    grove.count_empty_ground()
}