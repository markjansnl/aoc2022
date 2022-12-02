use day02::{input, RockPaperScissors};

fn total_score(input: &str) -> usize {
    input
        .lines()
        .map(|line| score(line[0..1].into(), line[2..3].into()))
        .sum()
}

fn score(opponent: RockPaperScissors, me: RockPaperScissors) -> usize {
    1 + me as usize + (4 + me as usize - opponent as usize) % 3 * 3
}

fn main() {
    println!("{}", total_score(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(15, total_score(input::EXAMPLE));
}

#[test]
fn test_score() {
    use RockPaperScissors::*;

    assert_eq!(1 + 3, score(Rock, Rock));
    assert_eq!(1 + 0, score(Paper, Rock));
    assert_eq!(1 + 6, score(Scissors, Rock));

    assert_eq!(2 + 6, score(Rock, Paper));
    assert_eq!(2 + 3, score(Paper, Paper));
    assert_eq!(2 + 0, score(Scissors, Paper));

    assert_eq!(3 + 0, score(Rock, Scissors));
    assert_eq!(3 + 6, score(Paper, Scissors));
    assert_eq!(3 + 3, score(Scissors, Scissors));
}
