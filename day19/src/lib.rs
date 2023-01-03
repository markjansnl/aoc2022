#![feature(iter_array_chunks)]

/*
 * This runs waayyy too long, so this doesn't work
 * Looked at reddit how to decrease the problem space, but took me too long to understand
 * So for this day only I runned the code of somebody else to get the stars.
 * 
 * Full credits to:
 * https://github.com/Crazytieguy/advent-of-code/tree/ae35e9347ee3fd2737f6c1ce291b74186cde11a1/2022/src/bin/day19
 */

use rayon::prelude::*;
use std::ops::{AddAssign, Add};

pub mod input;

#[derive(Debug, Default, Clone, Copy)]
pub struct Quantity {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Debug)]
pub struct Recipe {
    builds: Quantity,
    costs: Quantity,
}

#[derive(Debug)]
pub struct Blueprint {
    number: usize,
    recipes: [Recipe; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    robots: Quantity,
    inventory: Quantity,
}

// pub struct SubtractQuantityIterator {
//     quantity: Quantity,
//     subtract: Quantity,
// }

impl Quantity {
    #[inline]
    pub fn try_subtract(&self, rhs: &Self) -> Option<Self> {
        if self.ore >= rhs.ore
            && self.clay >= rhs.clay
            && self.obsidian >= rhs.obsidian
            && self.geode >= rhs.geode
        {
            Some(Quantity {
                ore: self.ore - rhs.ore,
                clay: self.clay - rhs.clay,
                obsidian: self.obsidian - rhs.obsidian,
                geode: self.geode - rhs.geode,
            })
        } else {
            None
        }
    }

    // pub fn subtract_iter(&self, rhs: &Self) -> SubtractQuantityIterator {
    //     SubtractQuantityIterator { quantity: *self, subtract: *rhs }
    // }

    #[inline]
    pub fn multiply(self, multiplier: usize) -> Self {
        Self {
            ore: self.ore * multiplier,
            clay: self.clay * multiplier,
            obsidian: self.obsidian * multiplier,
            geode: self.geode * multiplier,
        }
    }
}

impl Add for Quantity {
    type Output = Quantity;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Quantity {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl From<&str> for Blueprint {
    #[inline]
    fn from(line: &str) -> Self {
        let (number_str, recipes_str) = line.split_once(':').unwrap();
        Self {
            number: number_str[10..].parse().unwrap(),
            recipes: recipes_str
                .split_terminator('.')
                .enumerate()
                .map(|(index, recipe_str)| {
                    let splits = recipe_str.split(' ').collect::<Vec<_>>();
                    match index {
                        0 => Recipe {
                            builds: Quantity {
                                ore: 1,
                                ..Default::default()
                            },
                            costs: Quantity {
                                ore: splits[5].parse().unwrap(),
                                ..Default::default()
                            },
                        },
                        1 => Recipe {
                            builds: Quantity {
                                clay: 1,
                                ..Default::default()
                            },
                            costs: Quantity {
                                ore: splits[5].parse().unwrap(),
                                ..Default::default()
                            },
                        },
                        2 => Recipe {
                            builds: Quantity {
                                obsidian: 1,
                                ..Default::default()
                            },
                            costs: Quantity {
                                ore: splits[5].parse().unwrap(),
                                clay: splits[8].parse().unwrap(),
                                ..Default::default()
                            },
                        },
                        3 => Recipe {
                            builds: Quantity {
                                geode: 1,
                                ..Default::default()
                            },
                            costs: Quantity {
                                ore: splits[5].parse().unwrap(),
                                obsidian: splits[8].parse().unwrap(),
                                ..Default::default()
                            },
                        },
                        _ => unreachable!("Wrong input!"),
                    }
                })
                .array_chunks()
                .next()
                .unwrap(),
        }
    }
}

impl Blueprint {
    #[inline]
    pub fn quality_level(&self, minutes: usize) -> usize {
        let mut nodes = vec![Node {
            robots: Quantity {
                ore: 1,
                ..Default::default()
            },
            inventory: Default::default(),
        }];

        for minute in 0..minutes {
            if minute == 7 {
                let a = 1;
            }
            nodes = nodes
                .into_iter()
                .flat_map(|node| node.successors(self))
                .collect::<Vec<_>>();
            println!("{minute}: {nodes:?}");
            // println!("{minute}");
        }

        nodes
            .into_iter()
            .map(|node| node.inventory.geode)
            .max()
            .unwrap()
    }
}

impl Node {
    #[inline]
    pub fn successors(&self, blueprint: &Blueprint) -> impl Iterator<Item = Node> {
        let mut successors = vec![(*self, Quantity::default())];

        for recipe in blueprint.recipes.iter() {
            let mut extend_successors = Vec::new();
            let mut prev_inventory = self.inventory;
            let mut quantity = 0;
            while let Some(next_inventory) = prev_inventory.try_subtract(&recipe.costs) {
                quantity += 1;
                for (prev, built_robots) in &successors {
                    extend_successors.push((Node {
                        inventory: next_inventory,
                        robots: prev.robots,
                    }, *built_robots + recipe.builds.multiply(quantity)));
                }
                prev_inventory = next_inventory;
            }
            successors.extend(extend_successors.into_iter())
        }

        successors.into_iter().map(|(mut node, built_robots)| {
            node.inventory += node.robots;
            node.robots += built_robots;
            node
        })
    }
}

// impl Iterator for SubtractQuantityIterator {
//     type Item = Quantity;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(next) = self.quantity.try_subtract(&self.subtract) {
//             self.quantity = next;
//             Some(next)
//         } else {
//             None
//         }
//     }
// }

#[inline]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        // .par_bridge()
        .map(Blueprint::from)
        .map(|blueprint| blueprint.number * blueprint.quality_level(9))
        .sum()
}
