use day09::input;

fn count_tail_positions(input: &str) -> usize {
    day09::count_tail_positions(input)
}

fn main() {
    println!("{}", count_tail_positions(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(13, count_tail_positions(input::EXAMPLE1));
}
