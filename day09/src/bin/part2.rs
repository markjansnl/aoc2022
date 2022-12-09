use day09::input;

fn count_10th_tail_positions(input: &str) -> usize {
    day09::count_10th_tail_positions(input)
}

fn main() {
    println!("{}", count_10th_tail_positions(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(36, count_10th_tail_positions(input::EXAMPLE2));
}
