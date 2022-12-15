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

    pub fn new_with_bounds(sensor: Sensor, y: isize, bound: isize) -> Option<Self> {
        Self::new(sensor, y).map(|range_with_beacon| Self {
            range: *range_with_beacon.range.start().max(&0)
                ..=*range_with_beacon.range.end().min(&bound),
            beacons: HashSet::new(),
        })
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

    *(0..=bound)
        .into_par_iter()
        .filter_map(|y| {
            let ranges = sensors
                .iter()
                .filter_map(|sensor| RangeWithBeacon::new_with_bounds(*sensor, y, bound))
                .fold(Vec::<RangeWithBeacon>::new(), merge_ranges);

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
        .collect::<Vec<_>>()
        .first()
        .unwrap()
}
