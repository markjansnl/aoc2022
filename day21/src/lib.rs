use std::collections::HashMap;

pub mod input;

pub type MonkeyName = &'static str;
pub type Operator = &'static str;

pub struct Monkies(HashMap<MonkeyName, Yell>);

pub enum Yell {
    SpecificNumber(isize),
    MathOperation(MonkeyName, Operator, MonkeyName),
}

impl From<&'static str> for Monkies {
    fn from(input: &'static str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.split_once(": ")
                        .map(|(name, yell)| (name, yell.into()))
                        .unwrap()
                })
                .collect(),
        )
    }
}

impl From<&'static str> for Yell {
    fn from(yell: &'static str) -> Self {
        let splits = yell.split(' ').collect::<Vec<_>>();
        if splits.len() == 1 {
            Self::SpecificNumber(splits[0].parse().unwrap())
        } else {
            Self::MathOperation(splits[0], splits[1], splits[2])
        }
    }
}

impl Monkies {
    pub fn find(&self, name: MonkeyName) -> &'_ Yell {
        self.0.get(name).unwrap()
    }

    pub fn yell(&self, name: MonkeyName) -> isize {
        match self.find(name) {
            Yell::SpecificNumber(number) => *number,
            Yell::MathOperation(left, operator, right) => {
                let left_yell = self.yell(left);
                let right_yell = self.yell(right);
                match *operator {
                    "+" => left_yell + right_yell,
                    "-" => left_yell - right_yell,
                    "*" => left_yell * right_yell,
                    "/" => left_yell / right_yell,
                    _ => unreachable!("Wrong operator!"),
                }
            }
        }
    }

    pub fn has_humn(&self, name: MonkeyName) -> bool {
        if name == "humn" {
            true
        } else {
            match self.find(name) {
                Yell::SpecificNumber(_) => false,
                Yell::MathOperation(left, _, right) => self.has_humn(left) || self.has_humn(right),
            }
        }
    }

    pub fn calc_humn(&self, name: MonkeyName, result: isize) -> isize {
        if name == "humn" {
            result
        } else {
            match self.find(name) {
                Yell::SpecificNumber(_) => unreachable!(),
                Yell::MathOperation(left, operator, right) => {
                    let yell = self.yell(right);
                    if self.has_humn(left) {
                        match *operator {
                            "+" => self.calc_humn(left, result - yell),
                            "-" => self.calc_humn(left, result + yell),
                            "*" => self.calc_humn(left, result / yell),
                            "/" => self.calc_humn(left, result * yell),
                            _ => unreachable!("Wrong operator!"),
                        }
                    } else {
                        let yell = self.yell(left);
                        match *operator {
                            "+" => self.calc_humn(right, result - yell),
                            "-" => self.calc_humn(right, yell - result),
                            "*" => self.calc_humn(right, result / yell),
                            "/" => self.calc_humn(right, yell / result),
                            _ => unreachable!("Wrong operator!"),
                        }
                    }
                }
            }
        }
    }
}

pub fn root_number(input: &'static str) -> isize {
    let monkies = Monkies::from(input);
    monkies.yell("root")
}

pub fn humn_yell(input: &'static str) -> isize {
    let monkies = Monkies::from(input);
    if let Yell::MathOperation(left, _, right) = monkies.find("root") {
        if monkies.has_humn(left) {
            monkies.calc_humn(left, monkies.yell(right))
        } else {
            monkies.calc_humn(right, monkies.yell(left))
        }
    } else {
        0
    }
}

#[test]
fn test_part2() {
    let mut monkies = Monkies::from(input::USER);
    monkies
        .0
        .insert("humn", Yell::SpecificNumber(humn_yell(input::USER)));
    if let Yell::MathOperation(left, _, right) = monkies.find("root") {
        assert_eq!(monkies.yell(left), monkies.yell(right))
    }
}
