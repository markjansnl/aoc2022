use day13::{input, right_order_count};

fn main() {
    println!("{}", right_order_count(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(13, right_order_count(input::EXAMPLE));
}
