use day21::*;

fn main() {
    println!("{}", root_number(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(152, root_number(input::EXAMPLE));
}
