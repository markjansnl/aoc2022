#![feature(hash_drain_filter)]

use itertools::Itertools;
use std::{collections::HashMap, iter::Peekable, str::Chars};
pub mod input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub enum PathStep {
    Step,
    Rotate(Direction),
}

pub struct PathIterator {
    path: Peekable<Chars<'static>>,
    steps: u8,
    direction: Direction,
}

pub struct Board {
    lines: Vec<&'static [u8]>,
    path: PathIterator,
    position: Position,
    direction: Direction,
    edge_redirections: HashMap<(Position, Direction), (Position, Direction)>,
}

impl Position {
    #[inline]
    pub fn next(&self, direction: &Direction) -> Position {
        self.next_n(1, direction)
    }

    #[inline]
    pub fn next_n(&self, n: usize, direction: &Direction) -> Position {
        let Position { x, y } = *self;
        use Direction::*;
        match direction {
            Right => Position { x: x + n, y },
            Down => Position { x, y: y + n },
            Left => Position { x: x - n, y },
            Up => Position { x, y: y - n },
        }
    }
}

impl Direction {
    #[inline]
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }
}

impl From<&'static str> for PathIterator {
    #[inline]
    fn from(path_str: &'static str) -> Self {
        let mut iter = Self {
            path: path_str.chars().peekable(),
            steps: 0,
            direction: Direction::Right,
        };
        iter.parse_steps();
        iter
    }
}

impl Iterator for PathIterator {
    type Item = PathStep;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps > 0 {
            self.steps -= 1;
            Some(PathStep::Step)
        } else {
            if self.parse_rotation() {
                self.parse_steps();
                Some(PathStep::Rotate(self.direction))
            } else {
                None
            }
        }
    }
}

impl PathIterator {
    #[inline]
    fn parse_steps(&mut self) {
        self.steps = 0;
        while let Some(peeked) = self.path.peek() {
            if peeked == &'L' || peeked == &'R' {
                break;
            }
            self.steps = self.steps * 10 + (self.path.next().unwrap() as u8 - 48);
        }
    }

    #[inline]
    fn parse_rotation(&mut self) -> bool {
        use Direction::*;
        if let Some(c) = self.path.next() {
            self.direction = match c {
                'L' => match self.direction {
                    Right => Up,
                    Down => Right,
                    Left => Down,
                    Up => Left,
                },
                'R' => match self.direction {
                    Right => Down,
                    Down => Left,
                    Left => Up,
                    Up => Right,
                },
                _ => unreachable!("Wrong path input!"),
            };
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn wall_hitted(&mut self) {
        self.steps = 0;
    }
}

impl From<&'static str> for Board {
    #[inline]
    fn from(input: &'static str) -> Self {
        let (lines_str, path_str) = input.split_once("\n\n").unwrap();
        let lines: Vec<&'static [u8]> = lines_str.lines().map(|line| line.as_bytes()).collect();
        let position = Position {
            x: lines[0]
                .iter()
                .enumerate()
                .find(|(_, byte)| byte != &&b' ')
                .unwrap()
                .0
                + 1,
            y: 1,
        };
        Self {
            lines,
            path: PathIterator::from(path_str),
            position,
            direction: Direction::Right,
            edge_redirections: HashMap::new(),
        }
    }
}

impl Board {
    #[inline]
    pub fn walk_path(&mut self) {
        while let Some(step) = self.path.next() {
            match step {
                PathStep::Step => {
                    let (next_position, next_direction) = self.next_position();
                    if self.lines[next_position.y - 1][next_position.x - 1] == b'#' {
                        self.path.wall_hitted();
                    } else {
                        self.position = next_position;
                        self.direction = next_direction;
                    }
                }
                PathStep::Rotate(direction) => self.direction = direction,
            }
        }
    }

    #[inline]
    fn get(&self, position: &Position) -> u8 {
        if position.x > 0 && position.y > 0 {
            if let Some(line) = self.lines.get(position.y - 1) {
                if let Some(byte) = line.get(position.x - 1) {
                    return *byte;
                }
            }
        }
        return b' ';
    }

    #[inline]
    fn next_position(&self) -> (Position, Direction) {
        let mut next_position = self.position.next(&self.direction);
        if self.get(&next_position) != b' ' {
            return (next_position, self.direction);
        }

        if let Some((redirected_position, redirected_direction)) =
            self.edge_redirections.get(&(next_position, self.direction))
        {
            return (*redirected_position, *redirected_direction);
        }

        next_position = self.position;
        loop {
            next_position = next_position.next(&self.direction.opposite());
            if self.get(&next_position) != b' ' {
                continue;
            }
            return (next_position.next(&self.direction), self.direction);
        }
    }

    #[inline]
    pub fn final_password(&self) -> usize {
        self.position.y * 1000 + self.position.x * 4 + self.direction as usize
    }

    #[inline]
    pub fn fold_cube(&mut self, cube_width: usize) {
        let mut edges = (0..5)
            .map(|x| x * cube_width)
            .cartesian_product((0..5).map(|y| y * cube_width))
            .filter_map(|(x, y)| {
                let tiles = [
                    self.get(&Position { x, y }),
                    self.get(&Position { x: x + 1, y }),
                    self.get(&Position { x, y: y + 1 }),
                    self.get(&Position { x: x + 1, y: y + 1 }),
                ];
                let count = tiles.iter().filter(|byte| byte != &&b' ').count() as u8;
                if count > 0 {
                    Some((Position { x, y }, (tiles, count)))
                } else {
                    None
                }
            })
            .collect::<HashMap<Position, ([u8; 4], u8)>>();

        let inner_corners = edges.drain_filter(|_, (_, count)| count == &3).collect::<Vec<_>>();

        // For each inner corner, go connect edges until both edges are outer corners
        for (position, (tiles, count)) in inner_corners {
            println!("{position:?}");
            let mut prev_positions = vec![(position, (tiles, count))];
            let mut edges = edges.clone();
            loop {
                let mut next_positions = Vec::new();
                for (prev_position, (prev_tiles, prev_count)) in prev_positions {
                    let next_directions = Board::next_directions(&prev_tiles, prev_count)
                        .into_iter()
                        .map(|next_direction| prev_position.next_n(cube_width, &next_direction))
                        .filter(|next_position| {
                            edges.contains_key(next_position)
                        })
                        .collect::<Vec<_>>();
                    for next_position in next_directions {
                        next_positions.push((next_position, edges.remove(&next_position).unwrap()));
                    }
                }
                println!("    {next_positions:?}");
                if next_positions.iter().all(|(_, (_, count))| count == &1) {
                    break;
                }
                prev_positions = next_positions;
            }
        }
    }

    #[inline]
    fn next_directions(tiles: &[u8; 4], count: u8) -> [Direction; 2] {
        use Direction::*;
        if count == 1 {
            if tiles[0] != b' ' {
                [Up, Left]
            } else if tiles[1] != b' ' {
                [Up, Right]
            } else if tiles[2] != b' ' {
                [Down, Left]
            } else {
                [Down, Right]
            }
        } else if count == 2 {
            if tiles[0] == tiles[1] {
                [Left, Right]
            } else {
                [Up, Down]
            }
        } else {
            if tiles[0] == b' ' {
                [Up, Left]
            } else if tiles[1] == b' ' {
                [Up, Right]
            } else if tiles[2] == b' ' {
                [Down, Left]
            } else {
                [Down, Right]
            }
        }
    }
}

#[inline]
pub fn part1(input: &'static str) -> usize {
    let mut board = Board::from(input);
    board.walk_path();
    board.final_password()
}

#[inline]
pub fn part2(input: &'static str, cube_width: usize) -> usize {
    let mut board = Board::from(input);
    board.fold_cube(cube_width);
    board.walk_path();
    board.final_password()
}
