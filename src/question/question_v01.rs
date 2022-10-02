#![allow(non_snake_case)]
use crate::game::GameMode;
use crate::question::Question;
use crate::solution::Solution;
use rand::Rng;

pub struct QuestionV01 {
    x: i32,
    pub solution: Solution,
}

impl Question for QuestionV01 {
    fn variant(&self) -> GameMode {
        GameMode::GameModeV01
    }
    fn solution<'a>(&'a self) -> &'a Solution {
        &self.solution
    }
}

impl QuestionV01 {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Generate random number.
        let x: i32 = rng.gen_range(0..=25);
        // Generate solution.
        let solution = x.pow(2) as f64;
        let solution = Solution(solution);
        QuestionV01 { x, solution }
    }
}

impl ToString for QuestionV01 {
    fn to_string(&self) -> String {
        format!("{}^2 = ", self.x)
    }
}
