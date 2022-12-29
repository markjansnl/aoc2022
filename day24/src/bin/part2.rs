use day24::{input, part2};

fn main() {
    println!("{}", part2(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(54, part2(input::EXAMPLE));
}
