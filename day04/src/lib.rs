#![feature(iter_array_chunks)]

use std::ops::RangeInclusive;

pub mod input;

pub fn count<F: Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool>(
    input: &str,
    filter: F,
) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| elf.split_once('-').unwrap())
                .map(|(start, end)| start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .array_chunks()
                .next()
                .unwrap()
        })
        .filter(|[range1, range2]| filter(range1, range2) || filter(range2, range1))
        .count()
}
