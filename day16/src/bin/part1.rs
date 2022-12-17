use pathfinding::prelude::*;
use day16::*;

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
