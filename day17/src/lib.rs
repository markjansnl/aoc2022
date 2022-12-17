use std::{iter::repeat, collections::HashMap};

pub mod input;

#[derive(Clone, Copy)]
pub struct Row(u8);

pub struct Cave {
    rows: Vec<Row>,
    removed: usize,
    // rows_cache: HashMap<Vec<Row>, >
}

#[derive(Clone, Copy)]
pub struct Block([Row; 4]);

const BLOCKS: [Block; 5] = [
    // -
    Block([Row(0b0000000), Row(0b0000000), Row(0b0000000), Row(0b0011110)]),
    // +
    Block([Row(0b0000000), Row(0b0001000), Row(0b0011100), Row(0b0001000)]),
    // flipped L
    Block([Row(0b0000000), Row(0b0000100), Row(0b0000100), Row(0b0011100)]),
    // |
    Block([Row(0b0010000), Row(0b0010000), Row(0b0010000), Row(0b0010000)]),
    // block
    Block([Row(0b0000000), Row(0b0000000), Row(0b0011000), Row(0b0011000)]),
];

impl Row {
    #[inline]
    pub fn move_horizontal(&self, direction: char) -> Option<Row> {
        match direction {
            '<' => if self.0 & 0b01000000 == 0b01000000 { None } else { Some(Row(self.0 << 1)) },
            '>' => if self.0 & 0b00000001 == 0b00000001 { None } else { Some(Row(self.0 >> 1)) },
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
        Self{
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
        let mut remove = 0;
        for delta_y in 0..4 {
            if block.0[3 - delta_y].0 == 0 {
                return;
            }

            if let Some(cave_row) = self.rows.get_mut(y + delta_y) {
                cave_row.0 |= block.0[3 - delta_y].0;
                if cave_row.0 == 0b01111111 {
                    remove = y + delta_y;
                }
            } else {
                self.rows.push(block.0[3 - delta_y]);
            }
        }
        if remove > 0 {
            self.rows[1..].rotate_left(remove);
            self.rows.truncate(self.len() - remove);
            self.removed += remove;
        }
    }
}

#[inline]
pub fn height_after_block(n: usize, input: &str) -> usize {
    let mut directions = repeat(input.chars()).flatten();
    let blocks = repeat(BLOCKS.iter()).flatten().take(n);
    let mut cave = Cave::new();

    for next_block in blocks.into_iter() {
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
                break;
            }
            y -= 1;
        }
    }

    cave.len() - 1
}