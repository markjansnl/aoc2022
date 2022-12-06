use day05::{input, CrateMover9001};

fn top_crates(input: &str) -> String {
    day05::top_crates::<CrateMover9001>(input)
}

fn main() {
    println!("{}", top_crates(input::USER));
}

#[test]
fn test_example() {
    assert_eq!("MCD".to_string(), top_crates(input::EXAMPLE));
}
