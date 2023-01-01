use day16::{input, Cave};
use itertools::Itertools;
use rayon::prelude::*;

fn most_released_pressure(input: &'static str, me_opens: usize) -> usize {
    let cave = Cave::from(input);
    let valves = cave
        .all_destinations_with_flow
        .keys()
        .copied()
        .filter(|source| source != &"AA")
        .collect::<Vec<_>>();

    valves
        .iter()
        .copied()
        .powerset()
        .par_bridge()
        .filter(|destinations_me| destinations_me.len() == me_opens)
        .map(|destinations_me| {
            let destinations_elephant = valves
                .iter()
                .copied()
                .filter(|destination| !destinations_me.contains(destination))
                .collect::<Vec<_>>();
            released_pressure(destinations_me, &cave)
                + released_pressure(destinations_elephant, &cave)
        })
        .max()
        .unwrap()
}

fn released_pressure(destinations: Vec<&str>, cave: &Cave) -> usize {
    destinations
        .iter()
        .copied()
        .permutations(destinations.len())
        .map(|permutation| {
            let mut valve = "AA";
            let mut minute = 0;
            let mut releasing = 0;
            let mut released = 0;
            for destination in permutation.iter().copied() {
                for _step in cave
                    .all_destinations_with_flow
                    .get(valve)
                    .unwrap()
                    .get(&destination)
                    .unwrap()
                {
                    minute += 1;
                    released += releasing;
                    if minute == 26 {
                        return released;
                    }
                }
                valve = destination;
                minute += 1;
                released += releasing;
                if minute == 26 {
                    return released;
                }
                releasing += cave.flow.get(&destination).unwrap();
            }
            released + (26 - minute) * releasing
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", most_released_pressure(input::USER, 7));
}

#[test]
fn test_example() {
    assert_eq!(1707, most_released_pressure(input::EXAMPLE, 3));
}

#[test]
fn test_example_part2() {
    let cave = Cave::from(input::EXAMPLE);
    assert_eq!(
        1707,
        released_pressure(vec!["JJ", "BB", "CC"], &cave)
            + released_pressure(vec!["DD", "HH", "EE"], &cave)
    );
}
