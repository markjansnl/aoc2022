use day20::{input, part1};

fn main() {
    println!("{}", part1(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(3, part1(input::EXAMPLE));
}
