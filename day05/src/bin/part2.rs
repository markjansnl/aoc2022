use day05::{input, Stacks};

fn top_crates(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = Stacks::from(&mut lines);

    lines.next().unwrap();

    for line in lines {
        stacks.do_move_9001(line);
    }

    stacks.top_crates()
}

fn main() {
    println!("{}", top_crates(input::USER));
}

#[test]
fn test_example() {
    assert_eq!("MCD".to_string(), top_crates(input::EXAMPLE));
}
