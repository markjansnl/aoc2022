use day11::{input, Monkies};

fn monkey_business(input: &str) -> usize {
    let monkies = Monkies::from(input);
    for _round in 0..20 {
        monkies.round(true);
    }
    monkies.monkey_business()
}

fn main() {
    println!("{}", monkey_business(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(10605, monkey_business(input::EXAMPLE));
}
