use day15::{input, tuning_frequency};

fn main() {
    println!("{}", tuning_frequency(4000000, input::USER));
}

#[test]
fn test_example() {
    assert_eq!(56000011, tuning_frequency(20, input::EXAMPLE));
}
