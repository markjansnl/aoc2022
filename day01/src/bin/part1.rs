use day01::{aggregate_calories, input};

fn max_calories(input: &str) -> usize {
    aggregate_calories(input).max().unwrap()
}

fn main() {
    println!("{}", max_calories(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(24000, max_calories(input::EXAMPLE));
}
