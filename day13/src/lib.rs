pub mod input;

use rayon::prelude::*;
use serde::Deserialize;
use std::cmp::Ordering::*;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl From<&str> for Value {
    #[inline]
    fn from(line: &str) -> Self {
        serde_json::from_str::<Value>(line).unwrap()
    }
}

impl Ord for Value {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Value::*;
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (Number(_), List(b)) => cmp_list(&[self.clone()], b),
            (List(a), Number(_)) => cmp_list(a, &[other.clone()]),
            (List(a), List(b)) => cmp_list(a, b),
        }
    }
}

#[inline]
fn cmp_list(a: &[Value], b: &[Value]) -> std::cmp::Ordering {
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (_, 0) => Greater,
        (0, _) => Less,
        _ => {
            let (left_head, left_tail) = a.split_first().unwrap();
            let (right_head, right_tail) = b.split_first().unwrap();
            match left_head.cmp(right_head) {
                Equal => cmp_list(left_tail, right_tail),
                ordering => ordering,
            }
        }
    }
}

impl PartialOrd for Value {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[inline]
pub fn right_order_count(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .par_bridge()
        .filter_map(|(index, pair)| {
            pair.split_once('\n')
                .map(|(left, right)| (Value::from(left), Value::from(right)))
                .filter(|(left, right)| left < right)
                .map(|_| index + 1)
        })
        .sum()
}

#[inline]
pub fn decoder_key(input: &str) -> usize {
    let signal_2 = Value::from("[[2]]");
    let signal_6 = Value::from("[[6]]");

    let mut signals = input
        .lines()
        .par_bridge()
        .filter(|line| !line.is_empty())
        .map(Value::from)
        .collect::<Vec<_>>();

    signals.push(signal_2.clone());
    signals.push(signal_6.clone());

    signals.par_sort();

    signals
        .iter()
        .enumerate()
        .filter_map(|(index, signal)| (signal == &signal_2 || signal == &signal_6).then_some(index))
        .product()
}
