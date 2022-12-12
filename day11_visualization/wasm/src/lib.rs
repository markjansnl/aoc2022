#![allow(clippy::new_without_default)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn example() -> String {
    day11::input::EXAMPLE.to_string()
}

#[wasm_bindgen]
pub struct KeepAway {
    monkies: day11::Monkies,
    worry_level: usize,

    pub round: usize,
    pub monkey_index: usize,
}

#[wasm_bindgen]
impl KeepAway {
    #[wasm_bindgen(constructor)]
    pub fn new(input: String) -> Self {
        Self {
            monkies: input.as_str().into(),
            worry_level: 0,
            round: 1,
            monkey_index: 0,
        }
    }

    pub fn monkey_count(&self) -> usize {
        self.monkies.monkies.len()
    }

    pub fn monkey_items(&self, index: usize) -> Vec<usize> {
        self.monkies.monkies[index]
            .borrow()
            .items
            .iter()
            .map(|item| item.worry_level)
            .collect()
    }

    pub fn inspect(&mut self) -> usize {
        self.worry_level = self.monkies.monkies[self.monkey_index]
            .borrow_mut()
            .inspect()
            .unwrap()
            .worry_level;
        self.worry_level
    }

    pub fn get_borred(&mut self) -> usize {
        let mut item = day11::Item {
            worry_level: self.worry_level,
        };
        self.monkies.monkies[self.monkey_index]
            .borrow_mut()
            .get_borred(&mut item);
        self.worry_level = item.worry_level;
        self.worry_level
    }

    pub fn throw(&mut self) -> usize {
        let item = day11::Item {
            worry_level: self.worry_level,
        };
        let throw_to = self.monkies.monkies[self.monkey_index]
            .borrow()
            .test
            .get_throw_to(&item);
        self.monkies.monkies[throw_to].borrow_mut().catch(item);
        throw_to
    }

    pub fn next(&mut self) {
        self.monkey_index += 1;
        if self.monkey_index == self.monkies.monkies.len() {
            self.round += 1;
            self.monkey_index = 0;
        }
    }
}

#[test]
fn test_round() {
    let mut monkies = KeepAway::new(day11::input::EXAMPLE.to_string());

    assert_eq!(1501, monkies.inspect());
    assert_eq!(500, monkies.get_borred());
    assert_eq!(3, monkies.throw());

    assert_eq!(1862, monkies.inspect());
    assert_eq!(620, monkies.get_borred());
    assert_eq!(3, monkies.throw());

    monkies.next();

    assert_eq!(60, monkies.inspect());
    assert_eq!(20, monkies.get_borred());
    assert_eq!(0, monkies.throw());

    monkies.next();
    monkies.next();

    assert_eq!(77, monkies.inspect());
    assert_eq!(25, monkies.get_borred());
    assert_eq!(1, monkies.throw());

    assert_eq!(503, monkies.inspect());
    assert_eq!(167, monkies.get_borred());
    assert_eq!(1, monkies.throw());

    monkies.next();
    assert_eq!(2, monkies.round);

    assert_eq!(380, monkies.inspect());
}
