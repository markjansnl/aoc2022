pub mod input;

const GRID_SIZE: usize = 100 * 100;

pub struct Grid {
    size: isize,
    tree_length: [i8; GRID_SIZE],
    visible: [bool; GRID_SIZE],
    scenic_score: [isize; GRID_SIZE],
}

impl From<&str> for Grid {
    #[inline]
    fn from(input: &str) -> Self {
        let mut grid = Grid {
            size: 0,
            tree_length: [0; GRID_SIZE],
            visible: [false; GRID_SIZE],
            scenic_score: [1; GRID_SIZE],
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
            delta2: 1,
        }
    }

    #[inline]
    fn west(&self) -> Direction {
        Direction {
            start: self.size * self.size - 1,
            delta1: -1,
            delta2: -1,
        }
    }

    #[inline]
    fn south(&self) -> Direction {
        Direction {
            start: 0,
            delta1: self.size,
            delta2: -1 * (self.size - 1) * self.size + 1,
        }
    }

    #[inline]
    fn north(&self) -> Direction {
        Direction {
            start: self.size * self.size - 1,
            delta1: -1 * self.size,
            delta2: (self.size - 1) * self.size - 1,
        }
    }

    #[inline]
    pub fn check_visibility(&mut self) {
        self.check_visible(self.east());
        self.check_visible(self.west());
        self.check_visible(self.south());
        self.check_visible(self.north());
    }

    #[inline]
    fn check_visible(
        &mut self,
        Direction {
            start,
            delta1,
            delta2,
        }: Direction,
    ) {
        let mut index = start;
        let mut largest;
        for _ in 0..self.size {
            largest = -1;
            for _ in 0..self.size {
                let tree_length = self.tree_length[index as usize];
                if tree_length > largest {
                    largest = tree_length;
                    self.visible[index as usize] = true;
                }
                index += delta1;
            }
            index -= delta1;
            index += delta2;
        }
    }

    #[inline]
    pub fn count_visible(mut self) -> usize {
        self.check_visibility();
        self.visible.iter().filter(|visible| **visible).count()
    }

    #[inline]
    pub fn calc_scenic_score(&mut self) {
        self.calc_scenic_score_in_direction(self.east());
        self.calc_scenic_score_in_direction(self.west());
        self.calc_scenic_score_in_direction(self.south());
        self.calc_scenic_score_in_direction(self.north());
    }

    #[inline]
    fn calc_scenic_score_in_direction(
        &mut self,
        Direction {
            start,
            delta1,
            delta2,
        }: Direction,
    ) {
        let mut index = start;
        let mut last_tree_length;
        for _ in 0..self.size {
            last_tree_length = [0; 10];
            for row_index in 0..self.size {
                let tree_length = self.tree_length[index as usize];
                self.scenic_score[index as usize] *= row_index - last_tree_length[tree_length as usize];
                for last_tree_length_index in 0..=tree_length {
                    last_tree_length[last_tree_length_index as usize] = row_index;
                }
                index += delta1;
            }
            index -= delta1;
            index += delta2;
        }
    }

    #[inline]
    pub fn max_scenic_score(mut self) -> usize {
        self.calc_scenic_score();
        self.scenic_score.into_iter().max().unwrap() as usize
    }
}
