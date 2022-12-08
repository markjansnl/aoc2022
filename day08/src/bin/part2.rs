use day08::{input, Grid};

fn max_scenic_score(input: &str) -> usize {
    Grid::from(input).max_scenic_score()
}

fn main() {
    println!("{}", max_scenic_score(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(8, max_scenic_score(input::EXAMPLE));
}
