pub mod input;

pub fn aggregate_calories(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
}
