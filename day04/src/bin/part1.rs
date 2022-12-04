use std::ops::RangeInclusive;

use day04::{count, input};
use utils::RangeExt;

fn count_fully_contains(input: &str) -> usize {
    count(input, RangeInclusive::fully_contains)
}

fn main() {
    println!("{}", count_fully_contains(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(2, count_fully_contains(input::EXAMPLE));
}
