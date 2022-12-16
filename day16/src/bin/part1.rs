use pathfinding::prelude::*;
use std::collections::HashMap;
use day16::input;

pub type ValveId = String;
pub type Flow = usize;

#[derive(Debug, Default)]
pub struct Cave {
    flow: HashMap<ValveId, Flow>,
    destinations: HashMap<ValveId, Vec<ValveId>>,
    destinations_with_flow: HashMap<ValveId, HashMap<ValveId, Vec<ValveId>>>,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut cave = Self::default();
        for line in input.lines() {
            let (valve_str, destinations_str) = line
                .split_once(" valve ")
                .or(line.split_once(" valves "))
                .unwrap();
            let mut valve_splits = valve_str.split(&[' ', '=', ';'][..]);
            let valve_id = valve_splits.nth(1).unwrap().to_string();
            cave.flow.insert(
                valve_id.clone(),
                valve_splits.nth(3).unwrap().parse().unwrap(),
            );
            cave.destinations.insert(
                valve_id.clone(),
                destinations_str
                    .split(", ")
                    .map(ToString::to_string)
                    .collect(),
            );
        }

        for (valve_id, _) in cave
            .flow
            .iter()
            .filter(|(valve_id, flow)| valve_id.as_str() == "AA" || flow > &&0)
        {
            for (destination, _) in cave
                .flow
                .iter()
                .filter(|(destination, flow)| *destination != valve_id && flow > &&0)
            {
                let mut path = dijkstra(
                    valve_id,
                    |valve_id| {
                        cave.destinations
                            .get(valve_id)
                            .unwrap()
                            .iter()
                            .cloned()
                            .map(|destination| (destination, 1))
                    },
                    |valve_id| valve_id == destination,
                )
                .unwrap()
                .0;
                path.remove(0);
                if path.iter().take(path.len() - 1).find(|p| cave.flow.get(*p).unwrap() > &0).is_none() {
                    cave.destinations_with_flow
                        .entry(valve_id.clone())
                        .and_modify(|destinations_with_flow| {
                            destinations_with_flow.insert(destination.clone(), path.clone());
                        })
                        .or_insert([(destination.clone(), path)].into_iter().collect());
                }
            }
        }
        println!("{cave:#?}");
        cave
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Minute {
    minute: usize,
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
        let flow = cave.flow.get(&self.current_valve).unwrap();
        if !self.open_valves.contains(&self.current_valve) && flow > &0 {
            successors.push(self.open_valve(flow));
        }
        for (_, path) in cave
            .destinations_with_flow
            .get(&self.current_valve)
            .unwrap()
        {
            if path[0] != self.last_valve {
                let mut successor = self.clone();
                for destination in path.iter() {
                    successor = successor.move_to(destination.clone());
                    if successor.minute == 30 {
                        break;
                    }
                }
                successors.push(successor);
            }
        }
        successors
    }

    fn open_valve(&self, flow: &Flow) -> Minute {
        let mut next_minute = self.move_to(self.current_valve.clone());
        next_minute.open_valves.push(self.current_valve.clone());
        next_minute.releasing_pressure += flow;
        next_minute
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

    dfs_reach(minute_0, |minute| minute.successors(&cave))
        .filter(|minute| minute.minute == 30)
        .max_by_key(|minute| minute.released_pressure)
        .unwrap()
        .released_pressure
}

fn main() {
    // Does work with the smarter algorithm...
    println!("{}", most_released_pressure(input::USER));
}

#[test]
fn test_example() {
    // Fails with the smarter algorithm...
    assert_eq!(1651, most_released_pressure(input::EXAMPLE));
}
