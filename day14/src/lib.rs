use std::{collections::HashMap, fmt};

pub mod input;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

impl From<&str> for Position {
    fn from(position: &str) -> Self {
        let (x, y) = position.split_once(',').unwrap();
        Position {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Pixel {
    Rock,
    Sand,
    Abyss,
}

#[derive(Default)]
pub struct Scan {
    pixels: HashMap<Position, Pixel>,
}

pub struct LineIterator {
    start: Position,
    end: Position,
    done: bool,
}

impl LineIterator {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end,
            done: false,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.start;
        self.done = self.start == self.end;

        if self.start.x == self.end.x {
            self.start = Position {
                x: self.start.x,
                y: self.start.y + (self.end.y - self.start.y).signum(),
            };
        } else {
            self.start = Position {
                x: self.start.x + (self.end.x - self.start.x).signum(),
                y: self.start.y,
            };
        }

        Some(result)
    }
}

impl From<&str> for Scan {
    fn from(input: &str) -> Self {
        let mut scan = Scan::default();
        let mut max_y = 0;
        for line in input.lines() {
            line.split(" -> ").reduce(|start, end| {
                for position in LineIterator::new(start.into(), end.into()) {
                    scan.pixels.insert(position, Pixel::Rock);
                    max_y = max_y.max(position.y + 1);
                }
                end
            });
        }
        for position in LineIterator::new(
            Position { x: 0, y: max_y + 1 },
            Position {
                x: 1000,
                y: max_y + 1,
            },
        ) {
            scan.pixels.insert(position, Pixel::Abyss);
        }
        scan
    }
}

impl fmt::Debug for Scan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, max_y) =
            self.pixels
                .keys()
                .fold((isize::MAX, 0, 0), |(min_x, max_x, max_y), position| {
                    (
                        min_x.min(position.x),
                        max_x.max(position.x),
                        max_y.max(position.y),
                    )
                });
        for y in 0..=max_y {
            for x in min_x..=max_x {
                let _ = match self.pixels.get(&Position { x, y }) {
                    Some(Pixel::Rock) => write!(f, "#"),
                    Some(Pixel::Sand) => write!(f, "o"),
                    Some(Pixel::Abyss) => write!(f, "~"),
                    None => write!(f, "."),
                };
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

pub struct SandIterator<'s> {
    scan: &'s Scan,
    position: Position,
}

impl<'s> SandIterator<'s> {
    pub fn new(scan: &'s Scan) -> Self {
        Self {
            scan,
            position: Position { x: 500, y: 0 },
        }
    }
}

impl<'s> Iterator for SandIterator<'s> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.position;
        if self.scan.hit_test(&self.position).is_some() {
            return None;
        }

        next.y += 1;
        if self.scan.hit_test(&next).is_none() {
            self.position = next;
            return Some(self.position);
        }

        next.x -= 1;
        if self.scan.hit_test(&next).is_none() {
            self.position = next;
            return Some(self.position);
        }

        next.x += 2;
        if self.scan.hit_test(&next).is_none() {
            self.position = next;
            return Some(self.position);
        }

        None
    }
}

impl Scan {
    pub fn hit_test(&self, position: &Position) -> Option<Pixel> {
        self.pixels.get(position).copied()
    }
}

pub fn sand_count_before_abyss(input: &str) -> usize {
    let mut scan = Scan::from(input);

    for count in 0.. {
        let mut sand_iter = SandIterator::new(&scan);
        for _ in sand_iter.by_ref() {}
        if let Some(Pixel::Abyss) = scan.hit_test(&Position {
            x: sand_iter.position.x,
            y: sand_iter.position.y + 1,
        }) {
            return count;
        } else {
            scan.pixels.insert(sand_iter.position, Pixel::Sand);
        }
    }
    0
}

pub fn sand_count_before_full(input: &str) -> usize {
    let mut scan = Scan::from(input);

    for count in 0.. {
        let mut sand_iter = SandIterator::new(&scan);
        for _ in sand_iter.by_ref() {}
        if sand_iter.position == (Position { x: 500, y: 0 }) {
            return count + 1;
        } else {
            scan.pixels.insert(sand_iter.position, Pixel::Sand);
        }
    }
    0
}

#[test]
fn test_line_iterator() {
    let mut iter = LineIterator::new(Position { x: 0, y: 3 }, Position { x: 0, y: 5 });
    assert_eq!(Some(Position { x: 0, y: 3 }), iter.next());
    assert_eq!(Some(Position { x: 0, y: 4 }), iter.next());
    assert_eq!(Some(Position { x: 0, y: 5 }), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_sand_iterator() {
    let scan = Scan::from(input::EXAMPLE);

    let iter = SandIterator::new(&scan);
    for position in iter {
        println!("{position:?}");
    }
    // assert_eq!(Some(Position { x: 0, y: 3 }), iter.next());
    // assert_eq!(Some(Position { x: 0, y: 4 }), iter.next());
    // assert_eq!(Some(Position { x: 0, y: 5 }), iter.next());
    // assert_eq!(None, iter.next());
}
