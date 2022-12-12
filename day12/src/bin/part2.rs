use day12::{input, HeightMap};

fn fewest_steps(input: &str) -> usize {
    let height_map = HeightMap::from(input);
    height_map
        .squares()
        .into_iter()
        .filter(|(_, square)| square == &'a')
        .filter_map(|(start, _)| height_map.fewest_steps(&start))
        .min()
        .unwrap()
}

fn main() {
    println!("{}", fewest_steps(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(29, fewest_steps(input::EXAMPLE));
}
