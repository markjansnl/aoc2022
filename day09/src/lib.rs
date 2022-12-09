pub mod input;

mod motion;
mod rope;

pub use motion::*;
pub use rope::*;

pub fn count_tail_positions(input: &str, knots_count: usize) -> usize {
    let mut rope = Rope::new(input, knots_count);

    for _ in rope.by_ref() {}

    if cfg!(debug_assertions) {
        rope.print();
    }

    rope.count_tail_positions()
}
