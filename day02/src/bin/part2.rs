use day02::{input, RockPaperScissors};

fn total_score(input: &str) -> usize {
    input
        .lines()
        .map(|line| score(line[0..1].into(), line[2..3].into()))
        .sum()
}

fn score(opponent: RockPaperScissors, me: RockPaperScissors) -> usize {
    (2 + me as usize + opponent as usize) % 3 + 1 + me as usize * 3
}

fn main() {
    println!("{}", total_score(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(12, total_score(input::EXAMPLE));
}

#[test]
fn test_score() {
    use RockPaperScissors::*;

    assert_eq!(3 + 0, score(Rock, "X".into()));
    assert_eq!(1 + 0, score(Paper, "X".into()));
    assert_eq!(2 + 0, score(Scissors, "X".into()));

    assert_eq!(1 + 3, score(Rock, "Y".into()));
    assert_eq!(2 + 3, score(Paper, "Y".into()));
    assert_eq!(3 + 3, score(Scissors, "Y".into()));

    assert_eq!(2 + 6, score(Rock, "Z".into()));
    assert_eq!(3 + 6, score(Paper, "Z".into()));
    assert_eq!(1 + 6, score(Scissors, "Z".into()));
}
