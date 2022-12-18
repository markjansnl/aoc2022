use pathfinding::prelude::*;
use std::collections::HashSet;

pub mod input;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    #[inline]
    pub fn adjecent(&self) -> [Cube; 6] {
        [
            Cube {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Cube {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ]
    }
}

#[inline]
pub fn surface_area(input: &str, exclude_trapped: bool) -> usize {
    let cubes: HashSet<Cube> = input
        .lines()
        .map(|line| {
            let mut splits = line.split(',');
            Cube {
                x: splits.next().unwrap().parse().unwrap(),
                y: splits.next().unwrap().parse().unwrap(),
                z: splits.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) =
        (isize::MAX, 0, isize::MAX, 0, isize::MAX, 0);
    let mut all_sides = 0;
    let mut trapped_sides = 0;

    for cube in cubes.iter() {
        min_x = cube.x.min(min_x);
        max_x = cube.x.max(max_x);
        min_y = cube.y.min(min_y);
        max_y = cube.y.max(max_y);
        min_z = cube.z.min(min_z);
        max_z = cube.z.max(max_z);

        all_sides += cube
            .adjecent()
            .into_iter()
            .filter(|adjecent| !cubes.contains(adjecent))
            .count();
    }

    if exclude_trapped {
        let water = dfs_reach(
            Cube {
                x: min_x - 1,
                y: min_y - 1,
                z: min_z - 1,
            },
            |cube| {
                cube.adjecent()
                    .into_iter()
                    .filter(|adjecent| {
                        !cubes.contains(adjecent)
                            && (min_x - 1 <= adjecent.x && adjecent.x <= max_x + 1)
                            && (min_y - 1 <= adjecent.y && adjecent.y <= max_y + 1)
                            && (min_z - 1 <= adjecent.z && adjecent.z <= max_z + 1)
                    })
                    .collect::<Vec<_>>()
            },
        )
        .collect::<Vec<_>>();

        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let cube = Cube { x, y, z };
                    if !cubes.contains(&cube) && !water.contains(&cube) {
                        trapped_sides += cube
                            .adjecent()
                            .into_iter()
                            .filter(|adjecent| cubes.contains(adjecent))
                            .count();
                    }
                }
            }
        }
    }

    all_sides - trapped_sides
}
