use day12::{input, part2};

fn main() {
    println!("{}", part2(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(29, part2(input::EXAMPLE));
}
