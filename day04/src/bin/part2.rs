use day04::{count, input, Overlaps};

fn count_overlaps(input: &str) -> usize {
    count::<usize, _>(input, Overlaps::overlaps)
}

fn main() {
    println!("{}", count_overlaps(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(4, count_overlaps(input::EXAMPLE));
}
