use day06::{first_marker, input};

fn first_starter_marker(input: &str) -> usize {
    first_marker::<4>(input)
}

fn main() {
    println!("{}", first_starter_marker(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(7, first_starter_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, first_starter_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, first_starter_marker("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(
        10,
        first_starter_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    );
    assert_eq!(11, first_starter_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}
