use day04::{count, input, FullyContains};

fn count_fully_contains(input: &str) -> usize {
    count::<usize, _>(input, FullyContains::fully_contains)
}

fn main() {
    println!("{}", count_fully_contains(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(2, count_fully_contains(input::EXAMPLE));
}
