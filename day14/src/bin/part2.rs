use day14::{input, sand_count_before_full};

fn main() {
    println!("{}", sand_count_before_full(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(93, sand_count_before_full(input::EXAMPLE));
}
