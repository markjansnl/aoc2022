use day17::{input, height_after_block};

fn main() {
    println!("{}", height_after_block(1_000_000_000_000, input::USER));
}

#[test]
fn test_example() {
    assert_eq!(1_514_285_714_288, height_after_block(1_000_000_000_000, input::EXAMPLE));
}
