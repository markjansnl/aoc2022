use day07::{input, FileSystem};

fn sum_dirs_max_100_000(input: &str) -> usize {
    let file_system = FileSystem::from(input);
    file_system
        .iter()
        .filter(|node| node.is_directory())
        .map(|node| node.size(&file_system))
        .filter(|size| size < &100_000)
        .sum()
}

fn main() {
    println!("{}", sum_dirs_max_100_000(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(95437, sum_dirs_max_100_000(input::EXAMPLE));
}
