use std::{collections::HashSet, iter::repeat};

use crate::motion::*;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct Boundaries {
    left: isize,
    right: isize,
    top: isize,
    bottom: isize,
}

impl Boundaries {
    pub fn expand(&mut self, &Position { x, y }: &Position) -> Option<Direction> {
        if x < self.left {
            self.left = x;
            Some(Direction::Left)
        } else if x > self.right {
            self.right = x;
            Some(Direction::Right)
        } else if y < self.top {
            self.top = y;
            Some(Direction::Up)
        } else if y > self.bottom {
            self.bottom = y;
            Some(Direction::Down)
        } else {
            None
        }
    }
}

pub struct Rope {
    motions: MotionIterator,
    boundaries: Boundaries,
    knots: Vec<Position>,
    knot_index: usize,
    tail_positions: HashSet<Position>,
}

pub struct KnotMoveResult {
    pub index: usize,
    pub position: Position,
    pub boundary_expanded: Option<Direction>,
}

impl Iterator for Rope {
    type Item = KnotMoveResult;

    fn next(&mut self) -> Option<Self::Item> {
        let mut boundary_expanded = None;
        let i = self.knot_index;
        self.knot_index = (i + 1) % self.knots.len();

        if i == 0 {
            if let Some(direction) = self.motions.next() {
                self.knots[0] = self.knots[0].step(&direction);
                boundary_expanded = self.boundaries.expand(&self.knots[0]);
            } else {
                return None;
            }
        } else {
            let delta_x = self.knots[i].x - self.knots[i - 1].x;
            let delta_y = self.knots[i].y - self.knots[i - 1].y;
            if delta_x.abs() == 2 || delta_y.abs() == 2 {
                self.knots[i].x -= delta_x.signum();
                self.knots[i].y -= delta_y.signum();
                if i == self.knots.len() - 1 {
                    self.tail_positions.insert(self.knots[i]);
                }
            } else {
                return self.next();
            }
        }
        Some(KnotMoveResult {
            index: i,
            position: self.knots[i],
            boundary_expanded,
        })
    }
}

impl Rope {
    pub fn new(input: &str, knots_count: usize) -> Self {
        Self {
            motions: input.into(),
            boundaries: Boundaries::default(),
            knots: repeat(Position::default()).take(knots_count).collect(),
            knot_index: 0,
            tail_positions: [Position::default()].into_iter().collect(),
        }
    }

    pub fn count_tail_positions(&self) -> usize {
        self.tail_positions.len()
    }

    pub fn print(&self) {
        for y in self.boundaries.top..=self.boundaries.bottom {
            for x in self.boundaries.left..=self.boundaries.right {
                let position = Position { x, y };
                if x == 0 && y == 0 {
                    print!("s");
                } else {
                    let mut found = false;
                    for (index, knot) in self.knots.iter().enumerate() {
                        if &position == knot {
                            if index == 0 {
                                print!("H");
                            } else if index == self.knots.len() - 1 {
                                print!("T");
                            } else {
                                print!("{index}");
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        if self.tail_positions.contains(&position) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                }
            }
            println!();
        }
        println!();
    }
}
