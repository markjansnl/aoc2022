use day13::{input, Value};

fn decoder_key(input: &str) -> usize {
    let signal_2 = Value::from("[[2]]");
    let signal_6 = Value::from("[[6]]");

    let mut signals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Value::from)
        .collect::<Vec<_>>();

    signals.push(signal_2.clone());
    signals.push(signal_6.clone());

    signals.sort();

    signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| signal == &&signal_2 || signal == &&signal_6)
        .map(|(index, _)| index + 1)
        .product()
}

fn main() {
    println!("{}", decoder_key(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(140, decoder_key(input::EXAMPLE));
}
