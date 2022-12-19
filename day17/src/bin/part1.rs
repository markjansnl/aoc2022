use day17::{height_after_block, input};

fn main() {
    println!("{}", height_after_block(2022, input::USER));
}

#[test]
fn test_example() {
    assert_eq!(3068, height_after_block(2022, input::EXAMPLE));
}
