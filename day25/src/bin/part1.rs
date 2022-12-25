use day25::{input, sum};

fn main() {
    println!("{}", sum(input::USER));
}

#[test]
fn test_example() {
    assert_eq!("2=-1=0", sum(input::EXAMPLE).as_str());
}
