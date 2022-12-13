use day13::{input, Value};

fn right_order_count(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pair)| {
            pair.split_once('\n')
                .map(|(left, right)| (Value::from(left), Value::from(right)))
                .filter(|(left, right)| left < right)
                .map(|pair| (index, pair))
        })
        .map(|(index, _)| index + 1)
        .sum()
}

fn main() {
    println!("{}", right_order_count(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(13, right_order_count(input::EXAMPLE));
}
