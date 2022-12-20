use day16::*;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Minute {
    minute: usize,
    current_valve: [ValveId; 2],
    last_valve: [ValveId; 2],
    heading_to: [Vec<ValveId>; 2],
    open_valves: Vec<ValveId>,
    releasing_pressure: Flow,
    released_pressure: Flow,
    // backtrack: String,
}

const ME: usize = 0;
const ELEPHANT: usize = 1;

impl Minute {
    #[inline]
    pub fn successors(&self, cave: &Cave) -> Vec<Minute> {
        if self.minute == 26 {
            return Vec::new();
        }
        if self.open_valves.len() == cave.destinations_with_flow.keys().len() - 1 {
            return vec![Minute {
                minute: 26,
                released_pressure: self.released_pressure
                    + (26 - self.minute) * self.releasing_pressure,
                ..self.clone()
            }];
        }

        let me_successors = self.single_successors(cave, ME);
        let elephant_successors = self.single_successors(cave, ELEPHANT);

        me_successors
            .into_iter()
            .cartesian_product(elephant_successors.into_iter())
            .filter_map(|(me, elephant)| {
                let mut open_valves = self.open_valves.clone();
                if let Some(me_opened_valve) = me.open_valves.first() {
                    if let Some(elephant_opened_valve) = elephant.open_valves.first() {
                        if me_opened_valve == elephant_opened_valve {
                            return None;
                        }
                        open_valves.push(*elephant_opened_valve);
                    }
                    open_valves.push(*me_opened_valve);
                } else if let Some(elephant_opened_valve) = elephant.open_valves.first() {
                    open_valves.push(*elephant_opened_valve);
                }

                Some(Minute {
                    minute: self.minute + 1,
                    current_valve: [me.current_valve[ME], elephant.current_valve[ELEPHANT]],
                    last_valve: [me.last_valve[ME], elephant.last_valve[ELEPHANT]],
                    heading_to: [
                        me.heading_to[ME].clone(),
                        elephant.heading_to[ELEPHANT].clone(),
                    ],
                    open_valves: open_valves,
                    releasing_pressure: me.releasing_pressure
                        + elephant.releasing_pressure
                        + self.releasing_pressure,
                    released_pressure: self.released_pressure + self.releasing_pressure,
                    // backtrack: format!(
                    //     "{}\n{}, {}  {open_valves:?}",
                    //     self.backtrack, me.current_valve[ME], elephant.current_valve[ELEPHANT]
                    // ),
                })
            })
            .collect()
    }

    #[inline]
    fn single_successors(&self, cave: &Cave, who: usize) -> Vec<Minute> {
        if let Some(next_valve) = self.heading_to[who].first() {
            return vec![self.move_to(next_valve, who)];
        }
        let mut successors = Vec::new();
        let flow = cave.flow.get(&self.current_valve[who]).unwrap();
        if !self.open_valves.contains(&self.current_valve[who]) && flow > &0 {
            successors.push(self.open_valve(flow, who));
        }
        for path in cave
            .destinations_with_flow
            .get(&self.current_valve[who])
            .unwrap()
            .values()
        {
            let (first, tail) = path.split_first().unwrap();
            if *first != self.last_valve[who] {
                let mut successor = self.move_to(first, who);
                successor.heading_to[who] = tail.to_vec();
                successors.push(successor);
            }
        }
        successors
    }

    #[inline]
    fn open_valve(&self, flow: &Flow, who: usize) -> Minute {
        let mut next_minute = self.move_to(self.current_valve[who], who);
        next_minute.open_valves = vec![self.current_valve[who]];
        next_minute.releasing_pressure = *flow;
        next_minute
    }

    #[inline]
    fn move_to(&self, next_valve: ValveId, who: usize) -> Minute {
        let mut next_minute = self.clone();
        next_minute.current_valve[who] = next_valve;
        next_minute.last_valve[who] = self.current_valve[who];
        if !next_minute.heading_to[who].is_empty() {
            next_minute.heading_to[who].remove(0);
        }
        next_minute.open_valves = Vec::new();
        next_minute.releasing_pressure = 0;
        next_minute
    }
}

#[inline]
pub fn most_released_pressure(input: &'static str) -> usize {
    let cave = Cave::from(input);
    let minute_0 = Minute {
        current_valve: ["AA", "AA"],
        // backtrack: "AA, AA".to_string(),
        ..Default::default()
    };

    // dfs_reach(minute_0, |minute| minute.successors(&cave))
    //     .filter(|minute| minute.minute == 26)
    //     .max_by_key(|minute| minute.released_pressure)
    //     // .map(|minute| {
    //     //     println!("{}", minute.backtrack);
    //     //     minute
    //     // })
    //     .unwrap()
    //     .released_pressure

    recursive_dfs(minute_0, &cave)
}

fn recursive_dfs(start: Minute, cave: &Cave) -> usize {
    start
        .successors(cave)
        .into_par_iter()
        .map(|successor| {
            if successor.minute == 26 {
                successor.released_pressure
            } else {
                recursive_dfs(successor, cave)
            }
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    println!("{}", most_released_pressure(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(1707, most_released_pressure(input::EXAMPLE));
}
