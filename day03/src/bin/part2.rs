use day03::input;
use std::collections::HashSet;

fn priority_sum(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.bytes().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|rucksacks| {
            if let [rucksack1, rucksack2, rucksack3] = rucksacks {
                rucksack1
                    .intersection(rucksack2)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(rucksack3)
                    .map(|&unique| {
                        if unique >= 97 {
                            unique as usize - 96
                        } else {
                            unique as usize - 38
                        }
                    })
                    .next()
                    .unwrap()
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("{}", priority_sum(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(70, priority_sum(input::EXAMPLE));
}
