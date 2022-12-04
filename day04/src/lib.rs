#![feature(iter_array_chunks)]

use std::fmt::Debug;
use std::{ops::RangeInclusive, str::FromStr};

pub mod input;

pub trait FullyContains {
    /**
     * Left hand side fully contains right hand side
     *
     * Example:
     * ```
     * use aoc04::FullyContains;
     *
     * let lhs = 1..=6;
     * assert_eq!(true, lhs.fully_contains(2..=4));
     * assert_eq!(false, lhs.fully_contains(4..=8));
     * assert_eq!(false, lhs.fully_contains(8..=9));
     * ```
     */
    fn fully_contains(&self, item: &Self) -> bool;
}

impl<Idx: PartialOrd<Idx>> FullyContains for RangeInclusive<Idx> {
    #[inline]
    fn fully_contains(&self, item: &Self) -> bool {
        self.contains(item.start()) && self.contains(item.end())
    }
}

pub trait Overlaps {
    /**
     * Left hand side overlaps right hand side with minimal one item
     *
     * Example:
     * ```
     * use aoc04::Overlaps;
     *
     * let lhs = 1..=6;
     * assert_eq!(true, lhs.overlaps(2..=4));
     * assert_eq!(true, lhs.overlaps(4..=8));
     * assert_eq!(false, lhs.overlaps(8..=9));
     * ```
     */
    fn overlaps(&self, item: &Self) -> bool;
}

impl<Idx: PartialOrd<Idx>> Overlaps for RangeInclusive<Idx> {
    #[inline]
    fn overlaps(&self, item: &Self) -> bool {
        self.contains(item.start()) || self.contains(item.end())
    }
}

pub fn count<Idx, F: Fn(&RangeInclusive<Idx>, &RangeInclusive<Idx>) -> bool>(
    input: &str,
    filter: F,
) -> usize
where
    Idx: FromStr,
    Idx::Err: Debug,
{
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| elf.split_once('-').unwrap())
                .map(|(start, end)| start.parse::<Idx>().unwrap()..=end.parse::<Idx>().unwrap())
                .array_chunks()
                .next()
                .unwrap()
        })
        .filter(|[range1, range2]| filter(range1, range2) || filter(range2, range1))
        .count()
}

#[test]
fn test_fully_contains() {
    let lhs = 1..=6;
    assert_eq!(true, lhs.fully_contains(&(2..=4)));
    assert_eq!(false, lhs.fully_contains(&(4..=8)));
    assert_eq!(false, lhs.fully_contains(&(8..=9)));
}

#[test]
fn test_overlaps() {
    let lhs = 1..=6;
    assert_eq!(true, lhs.overlaps(&(2..=4)));
    assert_eq!(true, lhs.overlaps(&(4..=8)));
    assert_eq!(false, lhs.overlaps(&(8..=9)));
}
