use day22::{input, part2};

fn main() {
    println!("{}", part2(input::USER, 50));
}

#[test]
fn test_example() {
    assert_eq!(5031, part2(input::EXAMPLE, 4));
}
