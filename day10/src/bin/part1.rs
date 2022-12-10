use day10::{input, CPU};

fn signal_strength_sum(input: &str) -> isize {
    let cpu: CPU = input.into();
    let mut signal_strength_iter = cpu.enumerate().map(|(cycle, x)| (cycle as isize + 1) * x);

    let mut sum = signal_strength_iter.nth(19).unwrap();
    for _ in 0..5 {
        sum += signal_strength_iter.nth(39).unwrap();
    }
    sum
}

fn main() {
    println!("{}", signal_strength_sum(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(13140, signal_strength_sum(input::EXAMPLE));
}
