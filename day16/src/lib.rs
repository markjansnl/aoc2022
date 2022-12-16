use pathfinding::prelude::*;
use std::collections::HashMap;

pub mod input;

pub type ValveId = String;
pub type Flow = usize;

#[derive(Debug, Default, Clone)]
pub struct Valve {
    flow: Flow,
    destinations: Vec<ValveId>,
}

#[derive(Debug, Default)]
pub struct Cave {
    valves: HashMap<ValveId, Valve>,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        Self {
            valves: input
                .lines()
                .map(|line| {
                    let (valve_str, destinations_str) = line
                        .split_once(" valve ")
                        .or(line.split_once(" valves "))
                        .unwrap();
                    let mut valve_splits = valve_str.split(&[' ', '=', ';'][..]);
                    (
                        valve_splits.nth(1).unwrap().to_string(),
                        Valve {
                            flow: valve_splits.nth(3).unwrap().parse().unwrap(),
                            destinations: destinations_str
                                .split(", ")
                                .map(ToString::to_string)
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Minute {
    minute: u8,
    current_valve: ValveId,
    last_valve: ValveId,
    open_valves: Vec<ValveId>,
    releasing_pressure: Flow,
    released_pressure: Flow,
}

impl Minute {
    pub fn successors(&self, cave: &Cave) -> Vec<Minute> {
        let mut successors = Vec::new();
        if self.minute == 30 {
            return successors;
        }
        let valve = cave.valves.get(&self.current_valve).unwrap();
        if !self.open_valves.contains(&self.current_valve) && valve.flow > 0 {
            let mut next_minute = self.move_to(self.current_valve.clone());
            next_minute.open_valves.push(self.current_valve.clone());
            next_minute.releasing_pressure += valve.flow;
            successors.push(next_minute);
        }
        for destination in valve.destinations.iter() {
            if destination != &self.last_valve {
                successors.push(self.move_to(destination.clone()));
            }
        }
        successors
    }

    fn move_to(&self, next_valve: ValveId) -> Minute {
        Self {
            minute: self.minute + 1,
            current_valve: next_valve,
            last_valve: self.current_valve.clone(),
            open_valves: self.open_valves.clone(),
            releasing_pressure: self.releasing_pressure,
            released_pressure: self.released_pressure + self.releasing_pressure,
        }
    }
}

pub fn most_released_pressure<'i>(input: &'i str) -> usize {
    let cave = Cave::from(input);
    let minute_0 = Minute {
        current_valve: "AA".to_string(),
        ..Default::default()
    };

    // astar(
    //     &minute_0,
    //     |minute| minute.successors(&cave),
    //     |_minute| 0,
    //     |minute| minute.minute == 30,
    // )
    // .map(|(minutes, _total_cost)| dbg!(minutes))
    // .unwrap()
    // .last()
    // .unwrap()
    // .released_pressure

    dfs_reach(minute_0, |minute| minute.successors(&cave))
        .max_by_key(|minute| minute.released_pressure).unwrap().released_pressure
}
