use day15::{input, nr_of_no_beacons_on_line};

fn main() {
    println!("{}", nr_of_no_beacons_on_line(2000000, input::USER));
}

#[test]
fn test_example() {
    assert_eq!(26, nr_of_no_beacons_on_line(10, input::EXAMPLE));
}
