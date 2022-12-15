#![allow(clippy::len_without_is_empty)]

use rayon::prelude::*;
use std::{collections::HashSet, ops::RangeInclusive};
use utils::RangeExt;

pub mod input;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Sensor {
    position: Position,
    closest_beacon: Position,
}

#[derive(Debug)]
pub struct Range {
    range: RangeInclusive<isize>,
}

#[derive(Debug)]
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
        let mut sensor = Sensor::default();
        sensor.position.x = splits.nth(3).unwrap().parse().unwrap();
        sensor.position.y = splits.nth(2).unwrap().parse().unwrap();
        sensor.closest_beacon.x = splits.nth(6).unwrap().parse().unwrap();
        sensor.closest_beacon.y = splits.nth(2).unwrap().parse().unwrap();
        sensor
    }
}

impl Sensor {
    pub fn range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let manhatten_distance = self.position.manhatten_distance(&self.closest_beacon);
        let horizontal = manhatten_distance - (self.position.y - y).abs();
        if horizontal > 0 {
            Some(self.position.x - horizontal..=self.position.x + horizontal)
        } else {
            None
        }
    }
}

impl Range {
    #[inline]
    pub fn new(sensor: Sensor, y: isize) -> Option<Self> {
        sensor.range(y).map(|range| Self { range })
    }

    pub fn new_with_bound(sensor: Sensor, y: isize, bound: isize) -> Option<Self> {
        sensor.range(y).map(|range| Self {
            range: *range.start().max(&0)..=*range.end().min(&bound),
        })
    }

    pub fn len(self) -> usize {
        (self.range.end() - *self.range.start() + 1) as usize
    }
}

impl Merge for Range {
    #[inline]
    fn merge(&mut self, other: &mut Self) -> bool {
        if self.range.fully_contains(&other.range) {
            true
        } else if other.range.fully_contains(&self.range) {
            self.range = other.range.clone();
            true
        } else if self.range.overlaps(&other.range) {
            self.range = *self.range.start().min(other.range.start())
                ..=*self.range.end().max(other.range.end());
            true
        } else if *self.range.end() + 1 == *other.range.start() {
            self.range = *self.range.start()..=*other.range.end();
            true
        } else if *self.range.start() == *other.range.end() + 1 {
            self.range = *other.range.start()..=*self.range.end();
            true
        } else {
            false
        }
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
            return merged.into_iter().fold(Vec::<M>::new(), merge_ranges::<M>);
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
        .fold(Vec::<RangeWithBeacon>::new(), merge_ranges)
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
                .fold(Vec::<Range>::new(), merge_ranges);

            if ranges.len() == 2 {
                Some(y + 4000000 * (ranges.first().unwrap().range.end() + 1))
            } else if *ranges.first().unwrap().range.start() == 1 {
                Some(0)
            } else if *ranges.last().unwrap().range.end() == bound - 1 {
                Some(y + 4000000 * bound)
            } else {
                None
            }
        })
        .find_any(Option::is_some)
        .unwrap()
        .unwrap()
}
