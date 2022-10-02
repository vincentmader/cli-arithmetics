#![allow(non_snake_case)]
use crate::game::GameMode;
use crate::question::Question;
use crate::solution::Solution;
use rand::Rng;

pub struct QuestionV02 {
    x: i32,
    y: i32,
    pub solution: Solution,
}

impl Question for QuestionV02 {
    fn variant(&self) -> GameMode {
        GameMode::GameModeV02
    }
    fn solution<'a>(&'a self) -> &'a Solution {
        &self.solution
    }
}

impl QuestionV02 {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Generate two random numbers.
        let x = rng.gen_range(0..=25);
        let y = rng.gen_range(0..=25);
        // Generate solution.
        let (X, Y) = (x as f64, y as f64);
        let solution = Solution(X * Y);
        QuestionV02 { x, y, solution }
    }
}

impl ToString for QuestionV02 {
    fn to_string(&self) -> String {
        format!("{} * {} = ", self.x, self.y,)
    }
}
