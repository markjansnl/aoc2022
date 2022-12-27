use day20::{input, part2};

fn main() {
    println!("{}", part2(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(1623178306, part2(input::EXAMPLE));
}
