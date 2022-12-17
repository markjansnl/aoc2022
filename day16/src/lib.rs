use pathfinding::prelude::*;
use std::collections::HashMap;

pub mod input;

pub type ValveId = String;
pub type Flow = usize;

#[derive(Debug, Default)]
pub struct Cave {
    pub flow: HashMap<ValveId, Flow>,
    pub destinations: HashMap<ValveId, Vec<ValveId>>,
    pub destinations_with_flow: HashMap<ValveId, HashMap<ValveId, Vec<ValveId>>>,
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