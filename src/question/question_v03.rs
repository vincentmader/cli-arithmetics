#![allow(non_snake_case)]
use crate::game::GameMode;
use crate::operation::Operation;
use crate::question::Question;
use crate::solution::Solution;
use rand::Rng;

pub struct QuestionV03 {
    x: i32,
    y: i32,
    operation: Operation,
    pub solution: Solution,
}

impl Question for QuestionV03 {
    fn variant(&self) -> GameMode {
        GameMode::GameModeV03
    }
    fn solution<'a>(&'a self) -> &'a Solution {
        &self.solution
    }
}

impl QuestionV03 {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Generate two random numbers.
        let x = rng.gen_range(0..=30);
        let y = rng.gen_range(0..=15);
        // Choose a mathematical operation randomly.
        let operation = rng.gen_range(0..4);
        let operation = match operation {
            0 => Operation::Addition,
            1 => Operation::Subtraction,
            2 => Operation::Multiplication,
            3 => Operation::Division,
            _ => panic!("This can't happen. (hard-coded)"),
        };
        // Generate solution.
        let (X, Y) = (x as f64, y as f64);
        let solution = match &operation {
            Operation::Addition => X + Y,
            Operation::Subtraction => X - Y,
            Operation::Multiplication => X * Y,
            Operation::Division => X / Y,
        };
        let solution = Solution(solution);
        QuestionV03 {
            x,
            y,
            operation,
            solution,
        }
    }
}

impl ToString for QuestionV03 {
    fn to_string(&self) -> String {
        format!("{} {} {} = ", self.x, self.operation.to_string(), self.y,)
    }
}
