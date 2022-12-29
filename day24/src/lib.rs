use pathfinding::prelude::*;
use std::{cell::RefCell, collections::HashMap};

pub mod input;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

type BlizzardMap = HashMap<Position, Vec<char>>;
pub struct Blizzards {
    minutes: RefCell<Vec<BlizzardMap>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Node {
    minute: usize,
    position: Position,
}

pub struct Map {
    blizzards: Blizzards,
    start: Position,
    goal: Position,
}

impl Blizzards {
    pub fn is_free(&self, minute: usize, position: &Position) -> bool {
        while self.minutes.borrow().len() - 1 < minute {
            self.advance();
        }
        !self.minutes.borrow()[minute].contains_key(position)
    }

    fn advance(&self) {
        let mut blizzard_map: BlizzardMap = HashMap::new();
        for (Position { x, y }, v) in self.minutes.borrow()[self.minutes.borrow().len() - 1].iter()
        {
            for c in v {
                let next_position = match c {
                    '^' => {
                        if y == &1 {
                            Position {
                                x: *x,
                                y: self.height,
                            }
                        } else {
                            Position { x: *x, y: y - 1 }
                        }
                    }
                    'v' => {
                        if y == &self.height {
                            Position { x: *x, y: 1 }
                        } else {
                            Position { x: *x, y: y + 1 }
                        }
                    }
                    '<' => {
                        if x == &1 {
                            Position {
                                x: self.width,
                                y: *y,
                            }
                        } else {
                            Position { x: x - 1, y: *y }
                        }
                    }
                    '>' => {
                        if x == &self.width {
                            Position { x: 1, y: *y }
                        } else {
                            Position { x: x + 1, y: *y }
                        }
                    }
                    _ => Position { x: *x, y: *y },
                };
                blizzard_map
                    .entry(next_position)
                    .and_modify(|v| v.push(*c))
                    .or_insert_with(|| vec![*c]);
            }
        }

        self.minutes.borrow_mut().push(blizzard_map);
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut blizzard_map: BlizzardMap = HashMap::new();
        let mut goal = Position { x: 0, y: 0 };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if y > 0 {
                    if x == 1 && c == '#' {
                        goal.y = y;
                    } else if y == goal.y && c == '.' {
                        goal.x = x
                    }
                }
                if c != '.' {
                    blizzard_map
                        .entry(Position { x, y })
                        .and_modify(|v| v.push(c))
                        .or_insert_with(|| vec![c]);
                }
            }
        }

        Self {
            blizzards: Blizzards {
                minutes: RefCell::new(vec![blizzard_map]),
                width: goal.x,
                height: goal.y - 1,
            },
            start: Position { x: 1, y: 0 },
            goal,
        }
    }
}

impl Map {
    pub fn steps_avoiding_blizzards(&self, start_minute: usize) -> usize {
        astar(
            &Node {
                minute: start_minute,
                position: self.start,
            },
            |node| self.successors(node),
            |node| self.heuristic(node),
            |node| self.success(node),
        )
        .unwrap()
        .1
    }

    pub fn steps_avoiding_blizzards_back(&self, start_minute: usize) -> usize {
        astar(
            &Node {
                minute: start_minute,
                position: self.goal,
            },
            |node| self.successors(node),
            |node| self.heuristic_back(node),
            |node| self.success_back(node),
        )
        .unwrap()
        .1
    }

    fn successors(&self, node: &Node) -> Vec<(Node, usize)> {
        let Position { x, y } = node.position;
        let mut successor_positions = vec![node.position];
        if x > 1 {
            successor_positions.push(Position { x: x - 1, y });
        }
        if x < self.blizzards.width {
            successor_positions.push(Position { x: x + 1, y });
        }
        if y > 1 || (y == 1 && x == self.start.x) {
            successor_positions.push(Position { x, y: y - 1 });
        }
        if y < self.blizzards.height || (y == self.blizzards.height && x == self.goal.x) {
            successor_positions.push(Position { x, y: y + 1 });
        }

        let minute = node.minute + 1;
        successor_positions
            .into_iter()
            .filter_map(|position| {
                self.blizzards
                    .is_free(minute, &position)
                    .then_some((Node { minute, position }, 1))
            })
            .collect()
    }

    fn heuristic(&self, node: &Node) -> usize {
        (self.goal.x - node.position.x) + (self.goal.y - node.position.y)
    }

    fn success(&self, node: &Node) -> bool {
        node.position == self.goal
    }

    fn heuristic_back(&self, node: &Node) -> usize {
        (node.position.x - self.start.x) + (node.position.y - self.start.y)
    }

    fn success_back(&self, node: &Node) -> bool {
        node.position == self.start
    }
}

pub fn part1(input: &str) -> usize {
    Map::from(input).steps_avoiding_blizzards(0)
}

pub fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let steps1 = map.steps_avoiding_blizzards(0);
    let steps2 = map.steps_avoiding_blizzards_back(steps1);
    let steps3 = map.steps_avoiding_blizzards(steps1 + steps2);
    steps1 + steps2 + steps3
}
