use day23::{input, part2};

fn main() {
    println!("{}", part2(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(20, part2(input::EXAMPLE));
}
