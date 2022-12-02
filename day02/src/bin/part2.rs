use day02::{input, RockPaperScissors, Outcome};

fn total_score(input: &str) -> usize {
    input
        .lines()
        .map(|line| score(line[0..1].into(), line[2..3].into()))
        .sum()
}

fn score(opponent: RockPaperScissors, me: Outcome) -> usize {
    let choice_score = (2 + me as usize + opponent as usize) % 3 + 1;
    let outcome_score = me as usize * 3;
    
    choice_score + outcome_score
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
    use Outcome::*;

    assert_eq!(3 + 0, score(Rock, Lose));
    assert_eq!(1 + 0, score(Paper, Lose));
    assert_eq!(2 + 0, score(Scissors, Lose));

    assert_eq!(1 + 3, score(Rock, Draw));
    assert_eq!(2 + 3, score(Paper, Draw));
    assert_eq!(3 + 3, score(Scissors, Draw));

    assert_eq!(2 + 6, score(Rock, Win));
    assert_eq!(3 + 6, score(Paper, Win));
    assert_eq!(1 + 6, score(Scissors, Win));
}
