use day22::{input, part1};

fn main() {
    println!("{}", part1(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(6032, part1(input::EXAMPLE));
}
