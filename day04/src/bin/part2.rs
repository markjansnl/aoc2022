use std::ops::RangeInclusive;

use day04::{count, input};
use utils::RangeExt;

fn count_overlaps(input: &str) -> usize {
    count(input, RangeInclusive::overlaps)
}

fn main() {
    println!("{}", count_overlaps(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(4, count_overlaps(input::EXAMPLE));
}
