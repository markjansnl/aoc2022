#![feature(iter_array_chunks)]

use day03::input;
use std::collections::HashSet;

fn priority_sum(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.bytes().collect::<HashSet<_>>())
        .array_chunks()
        .map(|[rucksack1, rucksack2, rucksack3]| {
            rucksack1
                .intersection(&rucksack2.intersection(&rucksack3).copied().collect())
                .map(|&unique| {
                    if unique >= 97 {
                        unique as usize - 96
                    } else {
                        unique as usize - 38
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}

fn main() {
    println!("{}", priority_sum(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(70, priority_sum(input::EXAMPLE));
}
