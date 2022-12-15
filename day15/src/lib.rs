#![allow(clippy::len_without_is_empty)]

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

pub struct RangeWithBeacon {
    range: RangeInclusive<isize>,
    beacons: HashSet<isize>,
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

impl RangeWithBeacon {
    #[inline]
    pub fn new(sensor: Sensor, y: isize) -> Option<Self> {
        let manhatten_distance = sensor.position.manhatten_distance(&sensor.closest_beacon);
        let horizontal = manhatten_distance - (sensor.position.y - y).abs();
        if horizontal < 0 {
            None
        } else {
            Some(Self {
                range: sensor.position.x - horizontal..=sensor.position.x + horizontal,
                beacons: if sensor.closest_beacon.y == y {
                    HashSet::from([sensor.closest_beacon.x])
                } else {
                    HashSet::new()
                },
            })
        }
    }

    #[inline]
    pub fn merge(&mut self, other: &mut Self) -> bool {
        if self.range.fully_contains(&other.range) {
            self.beacons.extend(&other.beacons);
            true
        } else if other.range.fully_contains(&self.range) {
            self.range = other.range.clone();
            self.beacons.extend(&other.beacons);
            true
        } else if self.range.overlaps(&other.range) {
            self.range = *self.range.start().min(other.range.start())
                ..=*self.range.end().max(other.range.end());
            self.beacons.extend(&other.beacons);
            true
        } else {
            false
        }
    }

    pub fn len(self) -> usize {
        (self.range.end() - *self.range.start() + 1 - self.beacons.len() as isize) as usize
    }
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
fn merge_ranges(
    mut merged: Vec<RangeWithBeacon>,
    mut next: RangeWithBeacon,
) -> Vec<RangeWithBeacon> {
    for prev in merged.iter_mut() {
        if prev.merge(&mut next) {
            return merged
                .into_iter()
                .fold(Vec::<RangeWithBeacon>::new(), merge_ranges);
        }
    }
    merged.push(next);
    merged
}
