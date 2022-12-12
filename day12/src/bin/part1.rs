use day12::{input, part1};

fn main() {
    println!("{}", part1(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(31, part1(input::EXAMPLE));
}
