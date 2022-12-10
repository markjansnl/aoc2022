use day10::{input, CPU};

fn signal_strength_sum(input: &str) -> String {
    let cpu: CPU = input.into();
    let mut signal_strength_iter = cpu.enumerate();
    let mut output = String::with_capacity(6 * 41);

    for _ in 0..6 {
        for cycle in 0..40 {
            let (_, strength) = signal_strength_iter.next().unwrap();
            if strength >= cycle - 1 && strength <= cycle + 1 {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    output
}

fn main() {
    println!("{}", signal_strength_sum(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(
        r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#,
        signal_strength_sum(input::EXAMPLE)
    );
}
