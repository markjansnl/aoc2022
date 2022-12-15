#![allow(clippy::len_without_is_empty)]

use rayon::prelude::*;
use std::collections::HashSet;

pub mod input;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy)]
pub struct Sensor {
    position: Position,
    closest_beacon: Position,
}

#[derive(Clone, Copy)]
pub struct Range {
    start: isize,
    end: isize,
}

pub struct RangeWithBeacon {
    range: Range,
    beacons: HashSet<isize>,
}

pub trait Merge {
    fn merge(&mut self, other: &mut Self) -> bool;
}

impl Position {
    #[inline]
    pub fn manhatten_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<&str> for Sensor {
    #[inline]
    fn from(line: &str) -> Self {
        let mut splits = line.split(&[' ', ',', '=', ':'][..]);
        Self {
            position: Position {
                x: splits.nth(3).unwrap().parse().unwrap(),
                y: splits.nth(2).unwrap().parse().unwrap(),
            },
            closest_beacon: Position {
                x: splits.nth(6).unwrap().parse().unwrap(),
                y: splits.nth(2).unwrap().parse().unwrap(),
            },
        }
    }
}

impl Range {
    #[inline]
    pub fn new(sensor: Sensor, y: isize) -> Option<Self> {
        let manhatten_distance = sensor.position.manhatten_distance(&sensor.closest_beacon);
        let horizontal = manhatten_distance - (sensor.position.y - y).abs();
        if horizontal > 0 {
            Some(Self {
                start: sensor.position.x - horizontal,
                end: sensor.position.x + horizontal,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn new_with_bound(sensor: Sensor, y: isize, bound: isize) -> Option<Self> {
        Self::new(sensor, y).map(|range| Self {
            start: range.start.max(0),
            end: range.end.min(bound),
        })
    }

    #[inline]
    pub fn len(self) -> usize {
        (self.end - self.start) as usize + 1
    }

    #[inline]
    pub fn contains(&self, n: isize) -> bool {
        self.start <= n && n <= self.end
    }

    #[inline]
    pub fn fully_contains(&self, other: &Self) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }

    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}

impl Merge for Range {
    #[inline]
    fn merge(&mut self, other: &mut Self) -> bool {
        if self.fully_contains(other) {
            // just do nothing
        } else if other.fully_contains(self) {
            *self = *other;
        } else if self.overlaps(other) {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
        } else if self.end + 1 == other.start {
            self.end = other.end;
        } else if self.start == other.end + 1 {
            self.start = other.start;
        } else {
            return false;
        }
        true
    }
}

impl RangeWithBeacon {
    #[inline]
    pub fn new(sensor: Sensor, y: isize) -> Option<Self> {
        Range::new(sensor, y).map(|range| Self {
            range,
            beacons: if sensor.closest_beacon.y == y {
                HashSet::from([sensor.closest_beacon.x])
            } else {
                HashSet::new()
            },
        })
    }

    #[inline]
    pub fn len(self) -> usize {
        self.range.len() - self.beacons.len()
    }
}

impl Merge for RangeWithBeacon {
    #[inline]
    fn merge(&mut self, other: &mut Self) -> bool {
        if self.range.merge(&mut other.range) {
            self.beacons.extend(&other.beacons);
            true
        } else {
            false
        }
    }
}

#[inline]
fn merge_ranges<M: Merge>(mut merged: Vec<M>, mut next: M) -> Vec<M> {
    for prev in merged.iter_mut() {
        if prev.merge(&mut next) {
            return merged.into_iter().fold(Vec::new(), merge_ranges);
        }
    }
    merged.push(next);
    merged
}

#[inline]
pub fn nr_of_no_beacons_on_line(y: isize, input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| RangeWithBeacon::new(Sensor::from(line), y))
        .fold(Vec::new(), merge_ranges)
        .into_iter()
        .map(RangeWithBeacon::len)
        .sum()
}

#[inline]
pub fn tuning_frequency(bound: isize, input: &str) -> isize {
    let sensors = input.lines().map(Sensor::from).collect::<Vec<_>>();

    (0..=bound)
        .into_par_iter()
        .map(|y| {
            let ranges = sensors
                .iter()
                .filter_map(|sensor| Range::new_with_bound(*sensor, y, bound))
                .fold(Vec::new(), merge_ranges);

            if ranges.len() == 2 {
                y + 4000000 * (ranges.first().unwrap().end + 1)
            } else if ranges.first().unwrap().start == 1 {
                y
            } else if ranges.last().unwrap().end == bound - 1 {
                y + 4000000 * bound
            } else {
                -1
            }
        })
        .find_any(|freq| freq >= &0)
        .unwrap()
}
