#![allow(non_snake_case)]
use crate::game::GameMode;
use crate::question::Question;
use crate::solution::Solution;
use rand::Rng;

pub struct QuestionV05 {
    x: i32,
    y: i32,
    pub solution: Solution,
}

impl Question for QuestionV05 {
    fn variant(&self) -> GameMode {
        GameMode::GameModeV05
    }
    fn solution<'a>(&'a self) -> &'a Solution {
        &self.solution
    }
}

impl QuestionV05 {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Generate two random numbers.
        let x = rng.gen_range(0..=10);
        let y = rng.gen_range(0..=5);
        // Generate solution.
        let (X, Y) = (x as f64, y as f64);
        let solution = X.powf(Y);
        let solution = Solution(solution);
        QuestionV05 { x, y, solution }
    }
}

impl ToString for QuestionV05 {
    fn to_string(&self) -> String {
        format!("{} ^ {} = ", self.x, self.y,)
    }
}
