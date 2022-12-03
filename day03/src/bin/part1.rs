use day03::input;
use std::collections::HashSet;

fn priority_sum(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            left.bytes()
                .collect::<HashSet<_>>()
                .intersection(&right.bytes().collect::<HashSet<_>>())
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
    assert_eq!(157, priority_sum(input::EXAMPLE));
}
