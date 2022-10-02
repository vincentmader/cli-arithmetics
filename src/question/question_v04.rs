#![allow(non_snake_case)]
use crate::game::GameMode;
use crate::question::Question;
use crate::solution::Solution;
use rand::Rng;

pub struct QuestionV04 {
    x: i32,
    pub solution: Solution,
}

impl Question for QuestionV04 {
    fn variant(&self) -> GameMode {
        GameMode::GameModeV04
    }
    fn solution<'a>(&'a self) -> &'a Solution {
        &self.solution
    }
}

impl QuestionV04 {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // Generate two random numbers.
        let x = rng.gen_range(0..=10);
        // Generate solution.
        let solution = 2_f64.powf(x as f64);
        let solution = Solution(solution);
        QuestionV04 { x, solution }
    }
}

impl ToString for QuestionV04 {
    fn to_string(&self) -> String {
        format!("2^{} = ", self.x)
    }
}
