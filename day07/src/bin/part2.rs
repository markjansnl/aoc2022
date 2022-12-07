use day07::{input, FileSystem};

fn free_up_size(input: &str) -> usize {
    let file_system = FileSystem::from(input);
    let root_size = file_system.root().size(&file_system);
    file_system
        .iter()
        .filter(|node| node.is_directory())
        .map(|node| node.size(&file_system))
        .filter(|free_up| root_size - *free_up < 40_000_000)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", free_up_size(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(24933642, free_up_size(input::EXAMPLE));
}
