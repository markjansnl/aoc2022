use day13::{input, decoder_key};

fn main() {
    println!("{}", decoder_key(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(140, decoder_key(input::EXAMPLE));
}
