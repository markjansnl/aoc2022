#![feature(array_windows)]

use std::collections::HashSet;

pub mod input;

#[inline]
pub fn first_marker_iterators<const N: usize>(input: &str) -> usize {
    let mut set: HashSet<u8> = HashSet::with_capacity([(); N].len());
    input.bytes().collect::<Vec<_>>()[..]
        .array_windows::<N>()
        .enumerate()
        .find(|(_, window)| {
            set.clear();
            set.extend(window.iter());
            set.len() == window.len()
        })
        .map(|(index, window)| index + window.len())
        .unwrap()
}

#[inline]
pub fn first_marker<const N: usize>(input: &str) -> usize {
    let mut buffer = [0; N];
    let n = buffer.len();
    let mut buffer_index = 0;
    let mut map = [0; u8::MAX as usize];
    let mut map_len = 0;

    let mut bytes = input.bytes().enumerate();
    for _ in 0..n {
        let (index, byte) = bytes.next().unwrap();
        buffer[index] = byte;
        if map[byte as usize] == 0 {
            map_len += 1;
        }
        map[byte as usize] += 1;
    }

    for (index, byte) in bytes {
        if map_len == 1 {
            return index - 1;
        }

        let old_byte = buffer[buffer_index];
        map[old_byte as usize] -= 1;
        if map[old_byte as usize] == 0 {
            map_len -= 1;
        }

        buffer[buffer_index] = byte;
        buffer_index = (buffer_index + 1) % n;
        if map[byte as usize] == 0 {
            map_len += 1;
        }
        map[byte as usize] += 1;
    }
    0
}
