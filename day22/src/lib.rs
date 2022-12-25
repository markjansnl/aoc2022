use std::{iter::Peekable, str::Chars};

pub mod input;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
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
}

impl Position {
    #[inline]
    pub fn next(&self, direction: &Direction) -> Position {
        let Position { x, y } = *self;
        match direction {
            Direction::Right => Position { x: x + 1, y },
            Direction::Down => Position { x, y: y + 1 },
            Direction::Left => Position { x: x - 1, y },
            Direction::Up => Position { x, y: y - 1 },
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
        }
    }
}

impl Board {
    #[inline]
    pub fn walk_path(&mut self) {
        while let Some(step) = self.path.next() {
            match step {
                PathStep::Step => {
                    let next_position = self.next_position();
                    if self.lines[next_position.y - 1][next_position.x - 1] == b'#' {
                        self.path.wall_hitted();
                    } else {
                        self.position = next_position;
                    }
                }
                PathStep::Rotate(direction) => self.direction = direction,
            }
        }
    }

    #[inline]
    fn next_position(&self) -> Position {
        let mut next_position = self.position.next(&self.direction);
        if next_position.x > 0 && next_position.y > 0 {
            if let Some(line) = self.lines.get(next_position.y - 1) {
                if let Some(byte) = line.get(next_position.x - 1) {
                    if byte != &b' ' {
                        return next_position;
                    }
                }
            }
        }

        next_position = self.position;
        loop {
            next_position = next_position.next(&self.direction.opposite());
            if next_position.x > 0 && next_position.y > 0 {
                if let Some(line) = self.lines.get(next_position.y - 1) {
                    if let Some(byte) = line.get(next_position.x - 1) {
                        if byte != &b' ' {
                            continue;
                        }
                    }
                }
            }
            return next_position.next(&self.direction);
        }
    }

    #[inline]
    pub fn final_password(mut self) -> usize {
        self.walk_path();
        self.position.y * 1000 + self.position.x * 4 + self.direction as usize
    }
}

#[inline]
pub fn final_password(input: &'static str) -> usize {
    Board::from(input).final_password()
}