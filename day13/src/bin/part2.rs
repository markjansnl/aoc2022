use day13::{decoder_key, input};

fn main() {
    println!("{}", decoder_key(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(140, decoder_key(input::EXAMPLE));
}
