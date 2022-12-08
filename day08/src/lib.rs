use rayon::prelude::*;

pub mod input;

const GRID_SIZE: usize = 100 * 100;

pub struct Grid {
    size: isize,
    tree_length: [i8; GRID_SIZE],
}

impl From<&str> for Grid {
    #[inline]
    fn from(input: &str) -> Self {
        let mut grid = Grid {
            size: 0,
            tree_length: [0; GRID_SIZE],
        };

        let mut index = 0;
        for line in input.lines() {
            grid.size += 1;
            for byte in line.bytes() {
                grid.tree_length[index] = byte as i8 - 48;
                index += 1;
            }
        }
        grid
    }
}

struct Direction {
    start: isize,
    delta1: isize,
    delta2: isize,
}

impl Grid {
    #[inline]
    fn east(&self) -> Direction {
        Direction {
            start: 0,
            delta1: 1,
            delta2: 0,
        }
    }

    #[inline]
    fn west(&self) -> Direction {
        Direction {
            start: self.size * self.size - 1,
            delta1: -1,
            delta2: 0,
        }
    }

    #[inline]
    fn south(&self) -> Direction {
        Direction {
            start: 0,
            delta1: self.size,
            delta2: -self.size * self.size + 1,
        }
    }

    #[inline]
    fn north(&self) -> Direction {
        Direction {
            start: self.size * self.size - 1,
            delta1: -self.size,
            delta2: self.size * self.size - 1,
        }
    }

    #[inline]
    pub fn count_visible(mut self) -> usize {
        let mut visible = [false; GRID_SIZE];

        for direction in [self.east(), self.west(), self.south(), self.north()] {
            self.check_visible(direction, &mut visible)
        }

        visible.iter().filter(|visible| **visible).count()
    }

    #[inline]
    fn check_visible(
        &mut self,
        Direction {
            start,
            delta1,
            delta2,
        }: Direction,
        visible: &mut [bool; GRID_SIZE],
    ) {
        let mut index = start;
        let mut largest;
        for _ in 0..self.size {
            largest = -1;
            for _ in 0..self.size {
                let tree_length = self.tree_length[index as usize];
                if tree_length > largest {
                    largest = tree_length;
                    visible[index as usize] = true;
                }
                index += delta1;
            }
            index += delta2;
        }
    }

    #[inline]
    pub fn max_scenic_score(&mut self) -> usize {
        let scenic_scores = [self.east(), self.west(), self.south(), self.north()]
            .into_par_iter()
            .map(|direction| self.calc_scenic_score_in_direction(direction))
            .collect::<Vec<_>>();
        (0..self.size * self.size)
            .map(|index| {
                scenic_scores[0][index as usize]
                    * scenic_scores[1][index as usize]
                    * scenic_scores[2][index as usize]
                    * scenic_scores[3][index as usize]
            })
            .max()
            .unwrap() as usize
    }

    #[inline]
    fn calc_scenic_score_in_direction(
        &self,
        Direction {
            start,
            delta1,
            delta2,
        }: Direction,
    ) -> [isize; GRID_SIZE] {
        let mut index = start;
        let mut scenic_score = [1isize; GRID_SIZE];
        for _ in 0..self.size {
            let mut last_tree_length = [0; 10];
            for row_index in 0..self.size {
                let tree_length = self.tree_length[index as usize];
                scenic_score[index as usize] *= row_index - last_tree_length[tree_length as usize];
                for last_tree_length_index in 0..=tree_length {
                    last_tree_length[last_tree_length_index as usize] = row_index;
                }
                index += delta1;
            }
            index += delta2;
        }
        scenic_score
    }
}
