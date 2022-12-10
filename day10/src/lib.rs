pub mod input;

pub enum Instruction {
    Noop,
    AddX(isize),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        line.split_once(' ')
            .map(|(_, v)| Self::AddX(v.parse().unwrap()))
            .unwrap_or(Self::Noop)
    }
}

pub struct CPU {
    current_x: isize,
    cycles_next: u8,
    next_x: isize,
    instructions: Vec<Instruction>,
}

impl From<&str> for CPU {
    fn from(input: &str) -> Self {
        Self {
            current_x: 1,
            cycles_next: 0,
            next_x: 1,
            instructions: input.lines().map(|line| line.into()).collect(),
        }
    }
}

impl Iterator for CPU {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles_next == 0 {
            self.current_x = self.next_x;
            if self.instructions.is_empty() {
                return Some(self.current_x);
            }
            match self.instructions.remove(0) {
                Instruction::AddX(v) => {
                    self.cycles_next = 2;
                    self.next_x += v;
                }
                Instruction::Noop => {
                    self.cycles_next = 1;
                }
            }
        }
        self.cycles_next -= 1;
        Some(self.current_x)
    }
}

#[test]
fn test_iterator() {
    let mut cpu: CPU = "noop\naddx 3\naddx -5".into();
    assert_eq!(Some(1), cpu.next());
    assert_eq!(Some(1), cpu.next());
    assert_eq!(Some(1), cpu.next());
    assert_eq!(Some(4), cpu.next());
    assert_eq!(Some(4), cpu.next());
    assert_eq!(Some(-1), cpu.next());
    assert_eq!(Some(-1), cpu.next());
    assert_eq!(Some(-1), cpu.next());
    assert_eq!(Some(-1), cpu.next());
}
