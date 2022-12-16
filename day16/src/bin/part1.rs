use day16::{input, most_released_pressure};

fn main() {
    println!("{}", most_released_pressure(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(1651, most_released_pressure(input::EXAMPLE));
}
