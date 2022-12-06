#![feature(array_windows)]

use std::collections::HashSet;

pub mod input;

#[inline]
pub fn first_marker<const N: usize>(input: &str) -> usize {
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
