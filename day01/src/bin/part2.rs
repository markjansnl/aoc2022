use day01::{aggregate_calories, input};

fn max_calories(input: &str) -> usize {
    let mut aggregated_calories = aggregate_calories(input).collect::<Vec<_>>();
    aggregated_calories.sort();
    aggregated_calories.into_iter().rev().take(3).sum()
}

fn main() {
    println!("{}", max_calories(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(45000, max_calories(input::EXAMPLE));
}
