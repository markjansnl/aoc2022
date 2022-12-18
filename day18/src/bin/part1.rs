use day18::{input, surface_area};

fn main() {
    println!("{}", surface_area(input::USER, false));
}

#[test]
fn test_example() {
    assert_eq!(64, surface_area(input::EXAMPLE, false));
}
