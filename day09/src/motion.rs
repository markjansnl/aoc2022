#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => unreachable!("Wrong direction in input!"),
        }
    }
}

pub struct Motion {
    pub direction: Direction,
    pub steps: isize,
}

impl From<&str> for Motion {
    fn from(line: &str) -> Self {
        let (direction, steps) = line.split_once(' ').unwrap();
        Self {
            direction: direction.into(),
            steps: steps.parse().unwrap(),
        }
    }
}

pub struct MotionIterator {
    motions: Vec<Motion>,
}

impl From<&str> for MotionIterator {
    fn from(input: &str) -> Self {
        Self {
            motions: input.lines().map(|line| line.into()).collect()
        }
    }
}

impl Iterator for MotionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(motion) = self.motions.first_mut() {
            if motion.steps > 0 {
                motion.steps -= 1;
                Some(motion.direction)
            } else {
                self.motions.remove(0);
                self.next()
            }
        } else {
            None
        }
    }
}

#[test]
fn test_iterator() {
    let mut iter = MotionIterator::from(crate::input::EXAMPLE1);
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Up), iter.next());
    assert_eq!(Some(Direction::Up), iter.next());
    assert_eq!(Some(Direction::Up), iter.next());
    assert_eq!(Some(Direction::Up), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Down), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Down), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Left), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(Some(Direction::Right), iter.next());
    assert_eq!(None, iter.next());
}