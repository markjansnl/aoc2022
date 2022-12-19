use pathfinding::prelude::*;
use std::collections::HashMap;

pub mod input;

pub type ValveId = &'static str;
pub type Flow = usize;

#[derive(Debug, Default)]
pub struct Cave {
    pub flow: HashMap<ValveId, Flow>,
    pub destinations: HashMap<ValveId, Vec<ValveId>>,
    pub destinations_with_flow: HashMap<ValveId, HashMap<ValveId, Vec<ValveId>>>,
}

impl From<&'static str> for Cave {
    fn from(input: &'static str) -> Self {
        let mut cave = Self::default();
        for line in input.lines() {
            let (valve_str, destinations_str) = line
                .split_once(" valve ")
                .or_else(|| line.split_once(" valves "))
                .unwrap();
            let mut valve_splits = valve_str.split(&[' ', '=', ';'][..]);
            let valve_id = valve_splits.nth(1).unwrap();
            cave.flow
                .insert(valve_id, valve_splits.nth(3).unwrap().parse().unwrap());
            cave.destinations
                .insert(valve_id, destinations_str.split(", ").collect());
        }

        for (valve_id, _) in cave
            .flow
            .iter()
            .filter(|(valve_id, flow)| **valve_id == "AA" || flow > &&0)
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
                            .copied()
                            .map(|destination| (destination, 1))
                    },
                    |valve_id| valve_id == destination,
                )
                .unwrap()
                .0;
                path.remove(0);
                if !path
                    .iter()
                    .take(path.len() - 1)
                    .any(|p| cave.flow.get(*p).unwrap() > &0)
                {
                    cave.destinations_with_flow
                        .entry(valve_id)
                        .and_modify(|destinations_with_flow| {
                            destinations_with_flow.insert(destination, path.clone());
                        })
                        .or_insert_with(|| [(*destination, path)].into_iter().collect());
                }
            }
        }
        cave
    }
}
