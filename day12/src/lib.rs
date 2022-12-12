use pathfinding::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

pub mod input;

pub struct HeightMap {
    squares: HashMap<Position, Square>,
    start: Position,
    end: Position,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

pub type Square = char;

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let mut start = None;
        let mut end = None;
        let mut squares = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let position = Position {
                    x: x as isize,
                    y: y as isize,
                };
                match char {
                    'S' => {
                        start = Some(position);
                        squares.insert(position, 'a');
                    }
                    'E' => {
                        end = Some(position);
                        squares.insert(position, 'z');
                    }
                    _ => {
                        squares.insert(position, char);
                    }
                }
            }
        }

        Self {
            squares,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

impl HeightMap {
    pub fn start(&self) -> Position {
        self.start
    }

    pub fn squares(&self) -> Vec<(Position, Square)> {
        self.squares.clone().into_iter().collect()
    }

    pub fn fewest_steps(&self, start: &Position) -> Option<usize> {
        astar(
            start,
            |position| self.successors(position),
            |position| self.heuristic(position),
            |position| self.success(position),
        )
        .map(|(path, _)| path.len() - 1)
    }

    fn successors(&self, position: &Position) -> Vec<(Position, isize)> {
        [
            Position {
                x: position.x - 1,
                y: position.y,
            },
            Position {
                x: position.x + 1,
                y: position.y,
            },
            Position {
                x: position.x,
                y: position.y - 1,
            },
            Position {
                x: position.x,
                y: position.y + 1,
            },
        ]
        .into_iter()
        .filter_map(|successor| {
            self.squares
                .get(&successor)
                .filter(|&square| *square as u8 <= *self.squares.get(position).unwrap() as u8 + 1)
                .map(|_| (successor, 1))
        })
        .collect()
    }

    fn heuristic(&self, position: &Position) -> isize {
        (position.x - self.end.x).abs() + (position.y - self.end.y).abs()
    }

    fn success(&self, position: &Position) -> bool {
        position == &self.end
    }
}

pub fn part1(input: &str) -> usize {
    let height_map = HeightMap::from(input);
    height_map.fewest_steps(&height_map.start()).unwrap()
}

pub fn part2(input: &str) -> usize {
    let height_map = HeightMap::from(input);
    height_map
        .squares()
        .into_par_iter()
        .filter(|(_, square)| square == &'a')
        .filter_map(|(start, _)| height_map.fewest_steps(&start))
        .min()
        .unwrap()
}
