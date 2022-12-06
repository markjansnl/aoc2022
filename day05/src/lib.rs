#![feature(iter_array_chunks)]
#![allow(clippy::new_without_default)]

use std::{cell::RefCell, str::Lines};

pub mod input;

const NR_OF_STACKS: usize = 10;
const STACK_SIZE: usize = 100;
const INPUT_SIZE: usize = 10;

#[derive(Debug)]
pub struct Stacks([RefCell<Stack>; NR_OF_STACKS]);

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    insert_index: usize,
    stack_index: usize,
    stack: [u8; STACK_SIZE],
}

impl Stack {
    #[inline]
    pub fn new() -> Self {
        Stack {
            insert_index: INPUT_SIZE - 1,
            stack_index: INPUT_SIZE,
            stack: [0; STACK_SIZE],
        }
    }

    #[inline]
    pub fn insert(&mut self, byte: u8) {
        if byte != b' ' {
            self.stack[self.insert_index] = byte;
            self.insert_index -= 1;
        }
    }

    #[inline]
    pub fn push(&mut self, byte: u8) {
        self.stack[self.stack_index] = byte;
        self.stack_index += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> u8 {
        self.stack_index -= 1;
        self.stack[self.stack_index]
    }

    #[inline]
    pub fn popn_into(&mut self, n: usize, stack: &mut Stack) {
        self.stack_index -= n;
        stack.stack[stack.stack_index..stack.stack_index + n]
            .copy_from_slice(&self.stack[self.stack_index..self.stack_index + n]);
        stack.stack_index += n;
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        (self.stack_index - self.insert_index > 1).then_some(self.stack[self.stack_index - 1])
    }
}

type CrateMove = [usize; 3];

impl From<&mut Lines<'_>> for Stacks {
    #[inline]
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
            .fold(Stacks::new(), |stacks, (_, line_iter)| {
                for (stack_index, byte) in line_iter.enumerate() {
                    stacks.0[stack_index].borrow_mut().insert(byte);
                }
                stacks
            })
    }
}

impl Stacks {
    #[inline]
    pub fn new() -> Self {
        Stacks([
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
            RefCell::new(Stack::new()),
        ])
    }

    #[inline]
    pub fn move_crates_9000(&mut self, crate_move: CrateMove) {
        let [count, from, to] = crate_move;

        for _ in 0..count {
            self.0[to - 1]
                .borrow_mut()
                .push(self.0[from - 1].borrow_mut().pop());
        }
    }

    #[inline]
    pub fn move_crates_9001(&mut self, crate_move: CrateMove) {
        let [count, from, to] = crate_move;

        self.0[from - 1]
            .borrow_mut()
            .popn_into(count, &mut self.0[to - 1].borrow_mut());
    }

    #[inline]
    pub fn top_crates(&self) -> String {
        self.0
            .iter()
            .filter_map(|stack| stack.borrow().peek())
            .map(|byte| byte as char)
            .collect()
    }
}

#[inline]
fn parse_crate_move(line: &str) -> CrateMove {
    line.split(' ')
        .enumerate()
        .filter_map(|(i, s)| (i % 2 == 1).then(|| s.parse::<usize>().unwrap()))
        .array_chunks()
        .next()
        .unwrap()
}

pub trait CrateMover {
    fn move_crates(stacks: &mut Stacks, crate_move: CrateMove);
}

pub struct CrateMover9000 {}

impl CrateMover for CrateMover9000 {
    #[inline]
    fn move_crates(stacks: &mut Stacks, crate_move: CrateMove) {
        stacks.move_crates_9000(crate_move);
    }
}

pub struct CrateMover9001 {}

impl CrateMover for CrateMover9001 {
    #[inline]
    fn move_crates(stacks: &mut Stacks, crate_move: CrateMove) {
        stacks.move_crates_9001(crate_move);
    }
}

#[inline]
pub fn top_crates<CM: CrateMover>(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks = Stacks::from(&mut lines);

    lines.next().unwrap();

    for line in lines {
        CM::move_crates(&mut stacks, parse_crate_move(line));
    }

    stacks.top_crates()
}
