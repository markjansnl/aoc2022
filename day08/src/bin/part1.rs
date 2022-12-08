use day08::{input, Grid};

fn count_visible(input: &str) -> usize {
    Grid::from(input).count_visible()
}

fn main() {
    println!("{}", count_visible(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(21, count_visible(input::EXAMPLE));
}
