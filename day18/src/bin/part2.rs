use day18::{input, surface_area};

fn main() {
    println!("{}", surface_area(input::USER, true));
}

#[test]
fn test_example() {
    assert_eq!(58, surface_area(input::EXAMPLE, true));
}
