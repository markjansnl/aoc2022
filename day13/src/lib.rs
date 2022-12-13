pub mod input;

use serde::Deserialize;
use std::cmp::Ordering::*;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl From<&str> for Value {
    fn from(line: &str) -> Self {
        serde_json::from_str::<Value>(line).unwrap()
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Value::*;
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (Number(a), List(_)) => List(vec![Number(*a)]).cmp(other),
            (List(_), Number(b)) => self.cmp(&List(vec![Number(*b)])),
            (List(a), List(b)) => match (a.len(), b.len()) {
                (0, 0) => Equal,
                (_, 0) => Greater,
                (0, _) => Less,
                _ => {
                    let (left_head, left_tail) = a.split_first().unwrap();
                    let (right_head, right_tail) = b.split_first().unwrap();
                    match left_head.cmp(right_head) {
                        Equal => List(left_tail.to_vec()).cmp(&List(right_tail.to_vec())),
                        ordering => ordering,
                    }
                }
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
