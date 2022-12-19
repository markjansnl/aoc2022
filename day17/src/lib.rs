#![allow(clippy::len_without_is_empty, clippy::new_without_default)]

use std::{collections::HashMap, fmt, iter::repeat};

pub mod input;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Row(u8);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Cave {
    rows: Vec<Row>,
    removed: usize,
}

#[derive(Clone, Copy)]
pub struct Block([Row; 4]);

const BLOCKS: [Block; 5] = [
    // -
    Block([
        Row(0b0000000),
        Row(0b0000000),
        Row(0b0000000),
        Row(0b0011110),
    ]),
    // +
    Block([
        Row(0b0000000),
        Row(0b0001000),
        Row(0b0011100),
        Row(0b0001000),
    ]),
    // flipped L
    Block([
        Row(0b0000000),
        Row(0b0000100),
        Row(0b0000100),
        Row(0b0011100),
    ]),
    // |
    Block([
        Row(0b0010000),
        Row(0b0010000),
        Row(0b0010000),
        Row(0b0010000),
    ]),
    // block
    Block([
        Row(0b0000000),
        Row(0b0000000),
        Row(0b0011000),
        Row(0b0011000),
    ]),
];

impl Row {
    #[inline]
    pub fn move_horizontal(&self, direction: char) -> Option<Row> {
        match direction {
            '<' => {
                if self.0 & 0b01000000 == 0b01000000 {
                    None
                } else {
                    Some(Row(self.0 << 1))
                }
            }
            '>' => {
                if self.0 & 0b00000001 == 0b00000001 {
                    None
                } else {
                    Some(Row(self.0 >> 1))
                }
            }
            _ => unreachable!("Wrong direction!"),
        }
    }
}

impl Block {
    #[inline]
    pub fn move_horizontal(&self, direction: char) -> Option<Block> {
        if let Some(row0) = self.0[0].move_horizontal(direction) {
            if let Some(row1) = self.0[1].move_horizontal(direction) {
                if let Some(row2) = self.0[2].move_horizontal(direction) {
                    if let Some(row3) = self.0[3].move_horizontal(direction) {
                        return Some(Block([row0, row1, row2, row3]));
                    }
                }
            }
        }
        None
    }
}

impl Cave {
    #[inline]
    pub fn new() -> Self {
        Self {
            rows: vec![Row(0b01111111)],
            removed: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.rows.len() + self.removed
    }

    #[inline]
    pub fn hittest(&self, block: &Block, y: usize) -> bool {
        for delta_y in 0..4 {
            if let Some(cave_row) = self.rows.get(y + delta_y) {
                if cave_row.0 & block.0[3 - delta_y].0 != 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn place(&mut self, block: &Block, y: usize) {
        for delta_y in 0..4 {
            if block.0[3 - delta_y].0 == 0 {
                return;
            }

            if let Some(cave_row) = self.rows.get_mut(y + delta_y) {
                cave_row.0 |= block.0[3 - delta_y].0;
            } else {
                self.rows.push(block.0[3 - delta_y]);
            }
        }
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter().rev() {
            write!(f, "#")?;
            let mut mask = 64;
            while mask > 0 {
                if row.0 & mask == mask {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
                mask >>= 1;
            }
            writeln!(f, "#")?;
        }
        Ok(())
    }
}

#[inline]
pub fn height_after_block(n: usize, input: &str) -> usize {
    let mut directions = repeat(input.chars()).flatten();
    let blocks = repeat(BLOCKS.iter()).flatten().take(n);
    let mut cave = Cave::new();
    let mut cache = HashMap::new();

    for (block_nr, next_block) in blocks.into_iter().enumerate() {
        let mut block = *next_block;
        let mut y = cave.len() + 3;
        loop {
            if let Some(moved_block) = block.move_horizontal(directions.next().unwrap()) {
                if !cave.hittest(&moved_block, y) {
                    block = moved_block;
                }
            };
            if cave.hittest(&block, y - 1) {
                cave.place(&block, y);
                if cave.rows.len() > 100 {
                    if let Some((cached_block_nr, cached_len)) =
                        cache.get(&cave.rows[cave.rows.len() - 100..].to_vec())
                    {
                        let remaining_blocks = n - block_nr - 1;
                        let repeat_blocks = block_nr - cached_block_nr;
                        if remaining_blocks % repeat_blocks == 0 {
                            let repeat_len = cave.len() - cached_len;
                            return cave.len() - 1 + remaining_blocks / repeat_blocks * repeat_len;
                        }
                    }
                    cache.insert(
                        cave.rows[cave.rows.len() - 100..].to_vec(),
                        (block_nr, cave.len()),
                    );
                }
                break;
            }
            y -= 1;
        }
    }

    cave.len() - 1
}
