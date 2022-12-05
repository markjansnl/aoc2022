#![feature(iter_array_chunks)]
#![allow(clippy::new_without_default)]

use std::str::Lines;

pub mod input;

const NR_OF_STACKS: usize = 10;
const STACK_SIZE: usize = 100;
const INPUT_SIZE: usize = 10;

#[derive(Debug)]
pub struct Stacks([Stack; NR_OF_STACKS]);

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    insert_index: usize,
    stack_index: usize,
    stack: [u8; STACK_SIZE],
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            insert_index: INPUT_SIZE - 1,
            stack_index: INPUT_SIZE,
            stack: [0; STACK_SIZE],
        }
    }

    pub fn insert(&mut self, byte: u8) {
        if byte != b' ' {
            self.stack[self.insert_index] = byte;
            self.insert_index -= 1;
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.stack[self.stack_index] = byte;
        self.stack_index += 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.stack_index -= 1;
        self.stack[self.stack_index]
    }

    pub fn popn_into(&mut self, n: usize, stack: &mut Stack) {
        self.stack_index -= n;
        stack.stack[stack.stack_index..stack.stack_index + n]
            .copy_from_slice(&self.stack[self.stack_index..self.stack_index + n]);
        stack.stack_index += n;
    }

    pub fn peek(&self) -> Option<u8> {
        (self.stack_index - self.insert_index > 1).then_some(self.stack[self.stack_index - 1])
    }
}

impl From<&mut Lines<'_>> for Stacks {
    fn from(lines: &mut Lines<'_>) -> Self {
        lines
            .map(|line| {
                line.bytes()
                    .enumerate()
                    .filter_map(|(column, byte)| (column % 4 == 1).then_some(byte))
                    .peekable()
            })
            .map(|mut line_iter| (*line_iter.peek().unwrap(), line_iter))
            .take_while(|(first, _)| first != &b'1')
            .fold(Stacks::new(), |mut stacks, (_, line_iter)| {
                for (stack_index, byte) in line_iter.enumerate() {
                    stacks.0[stack_index].insert(byte);
                }
                stacks
            })
    }
}

impl Stacks {
    pub fn new() -> Self {
        Stacks([Stack::new(); NR_OF_STACKS])
    }

    pub fn do_move_9000(&mut self, line: &str) {
        let [count, from, to] = parse_move(line);

        for _ in 0..count {
            let byte = self.0[from - 1].pop();
            self.0[to - 1].push(byte);
        }
    }

    pub fn do_move_9001(&mut self, line: &str) {
        let [count, from, to] = parse_move(line);
        let mut temp_stack = Stack::new();

        self.0[from - 1].popn_into(count, &mut temp_stack);
        temp_stack.popn_into(count, &mut self.0[to - 1]);
    }

    pub fn top_crates(&self) -> String {
        self.0
            .iter()
            .filter_map(|stack| stack.peek())
            .map(|byte| byte as char)
            .collect()
    }
}

fn parse_move(line: &str) -> [usize; 3] {
    line.split(' ')
        .enumerate()
        .filter_map(|(i, s)| (i % 2 == 1).then(|| s.parse::<usize>().unwrap()))
        .array_chunks()
        .next()
        .unwrap()
}
