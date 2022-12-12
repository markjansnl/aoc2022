use day12::{input, HeightMap};

fn fewest_steps(input: &str) -> usize {
    let height_map = HeightMap::from(input);
    height_map.fewest_steps(&height_map.start()).unwrap()
}

fn main() {
    println!("{}", fewest_steps(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(31, fewest_steps(input::EXAMPLE));
}
