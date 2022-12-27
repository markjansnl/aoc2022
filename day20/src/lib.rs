pub mod input;

fn grove_coordinates(input: &str, decryption_key: isize, rounds: u8) -> isize {
    let sequence: Vec<isize> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * decryption_key)
        .collect();
    let mut mixed: Vec<usize> = (0..sequence.len()).collect();

    for _round in 0..rounds {
        for (seq_index, number) in sequence.iter().enumerate() {
            let index = mixed
                .iter()
                .enumerate()
                .find(|(_, i)| i == &&seq_index)
                .unwrap()
                .0;
            let next_index_isize = (index as isize + number) % (sequence.len() as isize - 1);
            let next_index = if next_index_isize < 0 {
                sequence.len() - 1 - (next_index_isize.unsigned_abs() % (sequence.len() - 1))
            } else {
                next_index_isize as usize
            };

            assert!(index < sequence.len());
            assert!(next_index < sequence.len());
            match next_index.cmp(&index) {
                std::cmp::Ordering::Greater => mixed[index..=next_index].rotate_left(1),
                std::cmp::Ordering::Less => mixed[next_index..=index].rotate_right(1),
                std::cmp::Ordering::Equal => {}
            }
        }
    }

    let seq_index0 = sequence
        .iter()
        .enumerate()
        .find(|(_, number)| number == &&0)
        .unwrap()
        .0;
    let index0 = mixed
        .iter()
        .enumerate()
        .find(|(_, index)| index == &&seq_index0)
        .unwrap()
        .0;
    sequence[mixed[(index0 + 1000) % sequence.len()]]
        + sequence[mixed[(index0 + 2000) % sequence.len()]]
        + sequence[mixed[(index0 + 3000) % sequence.len()]]
}

pub fn part1(input: &str) -> isize {
    grove_coordinates(input, 1, 1)
}

pub fn part2(input: &str) -> isize {
    grove_coordinates(input, 811589153, 10)
}
