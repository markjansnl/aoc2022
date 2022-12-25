use day22::{input, final_password};

fn main() {
    println!("{}", final_password(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(6032, final_password(input::EXAMPLE));
}
