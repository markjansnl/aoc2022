use day06::{first_marker, input};

fn first_starter_marker(input: &str) -> usize {
    first_marker::<14>(input)
}

fn main() {
    println!("{}", first_starter_marker(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(19, first_starter_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, first_starter_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, first_starter_marker("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(
        29,
        first_starter_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    );
    assert_eq!(26, first_starter_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}
