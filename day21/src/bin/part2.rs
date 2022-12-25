use day21::*;

fn main() {
    // 6915836165295: too high
    println!("{}", humn_yell(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(301, humn_yell(input::EXAMPLE));
}
