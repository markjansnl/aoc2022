use day14::{input, sand_count_before_abyss};

fn main() {
    println!("{}", sand_count_before_abyss(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(24, sand_count_before_abyss(input::EXAMPLE));
}
