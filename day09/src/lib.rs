pub mod input;
mod motion;

use std::collections::HashSet;

pub use motion::*;

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Up => Position { x: self.x, y: self.y - 1 },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
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

#[derive(Default)]
pub struct Rope {
    boundaries: Boundaries,
    head: Position,
    tail: Position,
    tail_positions: HashSet<Position>,
}

impl Rope {
    pub fn new() -> Self {
        let mut rope = Rope::default();
        rope.tail_positions.insert(rope.tail);
        rope
    }

    pub fn expand(&mut self, motion_direction: &Direction) -> Option<Direction> {
        self.boundaries.expand(&self.head.step(motion_direction))
    }

    pub fn move_head(&mut self, motion_direction: &Direction) -> Position {
        self.head = self.head.step(motion_direction);
        self.head
    }

    pub fn set_head(&mut self, position: &Position) {
        self.boundaries.expand(position);
        self.head = *position;
    }

    pub fn move_tail(&mut self) -> Option<Position> {
        let delta_x = self.tail.x - self.head.x;
        let delta_y = self.tail.y - self.head.y;
        if delta_x.abs() == 2 || delta_y.abs() == 2 {
            self.tail = Position { x: self.tail.x - delta_x.signum(), y: self.tail.y - delta_y.signum() };
            self.tail_positions.insert(self.tail);
            Some(self.tail)
        } else {
            None
        }
    }

    pub fn apply_motion(&mut self, motion_direction: &Direction) {
        self.expand(&motion_direction);
        self.move_head(&motion_direction);
        self.move_tail();
    }

    pub fn count_tail_positions(&self) -> usize {
        self.tail_positions.len()
    }

}

pub fn _print_ropes(ropes: &[Rope]) {
    for y in ropes[0].boundaries.top..=ropes[0].boundaries.bottom {
        for x in ropes[0].boundaries.left..=ropes[0].boundaries.right {
            let position = Position { x, y };
            if x == 0 && y == 0 {
                print!("s");
            } else {
                let mut found = false;
                for (index, rope) in ropes.iter().enumerate() {
                    if position == rope.head {
                        if index == 0 {
                            print!("H");
                        } else {
                            print!("{index}");
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    if position == ropes[8].tail {
                        print!("T");
                    } else if ropes[8].tail_positions.contains(&position) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
        println!();
    }
}

pub fn count_tail_positions(input: &str) -> usize {
    let mut rope = Rope::new();
    let motions = MotionIterator::from(input);

    for motion_direction in motions {
        rope.apply_motion(&motion_direction);
    }

    rope.count_tail_positions()
}

pub fn count_10th_tail_positions(input: &str) -> usize {
    let mut ropes = [
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
        Rope::new(),
    ];
    let motions = MotionIterator::from(input);

    for motion_direction in motions {
        ropes[0].expand(&motion_direction);
        ropes[0].move_head(&motion_direction);
        ropes[0].move_tail();
        ropes.iter_mut().reduce(|prev, next| {
            next.set_head(&prev.tail);
            next.move_tail();
            next
        }).unwrap();
    }

    ropes[8].count_tail_positions()
}