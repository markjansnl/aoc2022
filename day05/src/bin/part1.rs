use day05::{input, CrateMover9000};

fn top_crates(input: &str) -> String {
    day05::top_crates::<CrateMover9000>(input)
}

fn main() {
    println!("{}", top_crates(input::USER));
}

#[test]
fn test_example() {
    assert_eq!("CMZ".to_string(), top_crates(input::EXAMPLE));
}
