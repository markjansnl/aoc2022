use day03::input;
use std::collections::HashSet;

fn priority_sum(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line[..line.len() / 2]
                .bytes()
                .collect::<HashSet<_>>()
                .intersection(&line[line.len() / 2..].bytes().collect())
                .map(|&dup| (dup as usize - 38) % 58)
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
