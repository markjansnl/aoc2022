use std::cell::RefCell;

pub mod input;

pub struct Monkies {
    pub monkies: Vec<RefCell<Monkey>>,
    divisible_by_product: usize,
}

pub struct Monkey {
    pub items: Vec<Item>,
    items_inspected: usize,
    operation: Operation,
    pub test: Test,
}

#[derive(Clone, Copy)]
pub struct Item {
    pub worry_level: usize,
}

pub struct Operation {
    operator: Operator,
    operand: Operand,
}

pub enum Operator {
    Add,
    Multiply,
}

pub enum Operand {
    Old,
    Value(usize),
}

pub struct Test {
    divisible_by: usize,
    monkey_true: usize,
    monkey_false: usize,
}

impl From<&str> for Monkies {
    fn from(input: &str) -> Self {
        let mut divisible_by_product = 1;
        let monkies = input
            .split("\n\n")
            .map(|monkey_input| {
                let monkey = Monkey::from(monkey_input);
                divisible_by_product *= monkey.test.divisible_by;
                RefCell::new(monkey)
            })
            .collect::<Vec<_>>();

        Monkies {
            monkies,
            divisible_by_product,
        }
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next().unwrap();
        Monkey {
            items: lines.next().unwrap()[18..]
                .split(", ")
                .map(|item| item.into())
                .collect(),
            items_inspected: 0,
            operation: lines.next().unwrap().into(),
            test: [
                lines.next().unwrap(),
                lines.next().unwrap(),
                lines.next().unwrap(),
            ]
            .into(),
        }
    }
}

impl From<&str> for Item {
    fn from(input: &str) -> Self {
        Item {
            worry_level: input.parse().unwrap(),
        }
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        Operation {
            operator: input[23..24].into(),
            operand: input[25..].into(),
        }
    }
}

impl From<&str> for Operator {
    fn from(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => unreachable!("Wrong operator input!"),
        }
    }
}

impl From<&str> for Operand {
    fn from(input: &str) -> Self {
        match input {
            "old" => Self::Old,
            value => Self::Value(value.parse().unwrap()),
        }
    }
}

impl From<[&str; 3]> for Test {
    fn from(input: [&str; 3]) -> Self {
        Self {
            divisible_by: input[0][21..].parse().unwrap(),
            monkey_true: input[1][29..].parse().unwrap(),
            monkey_false: input[2][30..].parse().unwrap(),
        }
    }
}

impl Monkies {
    pub fn round(&self, get_borred: bool) {
        for monkey in &self.monkies {
            let mut monkey = monkey.borrow_mut();
            while let Some(mut item) = monkey.inspect() {
                if get_borred {
                    monkey.get_borred(&mut item);
                } else {
                    self.manage_worry_levels(&mut item);
                }
                let throw_to = monkey.test.get_throw_to(&item);
                self.monkies[throw_to].borrow_mut().catch(item);
            }
        }
    }

    pub fn manage_worry_levels(&self, item: &mut Item) {
        item.worry_level %= self.divisible_by_product;
    }

    pub fn monkey_business(&self) -> usize {
        let mut items_inspected = self
            .monkies
            .iter()
            .map(|monkey| monkey.borrow().items_inspected)
            .collect::<Vec<_>>();
        items_inspected.sort();
        items_inspected.iter().rev().take(2).product()
    }
}

impl Monkey {
    pub fn inspect(&mut self) -> Option<Item> {
        if !self.items.is_empty() {
            let mut item = self.items.remove(0);
            self.operation.apply(&mut item);
            self.items_inspected += 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn get_borred(&self, item: &mut Item) {
        item.worry_level /= 3;
    }

    pub fn catch(&mut self, item: Item) {
        self.items.push(item);
    }
}

impl Operation {
    pub fn apply(&self, item: &mut Item) {
        let old = item.worry_level;
        let value = match self.operand {
            Operand::Old => old,
            Operand::Value(value) => value,
        };
        item.worry_level = match self.operator {
            Operator::Add => old + value,
            Operator::Multiply => old * value,
        };
    }
}

impl Test {
    pub fn get_throw_to(&self, item: &Item) -> usize {
        if item.worry_level % self.divisible_by == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        }
    }
}
