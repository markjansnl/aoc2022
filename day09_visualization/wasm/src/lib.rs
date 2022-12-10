use day09::Rope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RopeModel {
    rope: Rope
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

#[wasm_bindgen]
pub struct StepResult {
    pub index: usize,
    pub position: Position,
}

#[wasm_bindgen]
impl RopeModel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            rope: Rope::new(day09::input::EXAMPLE2, 10),
        }
    }

    pub fn step(&mut self) -> Option<StepResult> {
        if let Some(knot_move_result) = self.rope.next() {
            return Some(StepResult {
                index: knot_move_result.index,
                position: Position { x: knot_move_result.position.x, y: knot_move_result.position.y }
            })
        }
        None
    }

    pub fn count_tail_positions(&self) -> usize {
        self.rope.count_tail_positions()
    }
}