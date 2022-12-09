pub mod input;

mod motion;
mod rope;

pub use motion::*;
pub use rope::*;

pub fn count_tail_positions(input: &str, n: usize) -> usize {
    let mut rope = Rope::new(input, n);

    for _ in rope.by_ref() {}

    if cfg!(debug_assertions) {
        rope.print();
    }

    rope.count_tail_positions()
}
