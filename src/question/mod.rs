mod question_v01;
mod question_v02;
mod question_v03;
mod question_v04;
mod question_v05;
use crate::answer::Answer;
use crate::game::GameMode;
use crate::solution::Solution;
use crate::utils;
use colored::Colorize;
pub use question_v01::QuestionV01;
pub use question_v02::QuestionV02;
pub use question_v03::QuestionV03;
pub use question_v04::QuestionV04;
pub use question_v05::QuestionV05;
use std::io::Write;

pub trait Question: ToString {
    fn variant(&self) -> GameMode;

    fn solution<'a>(&'a self) -> &'a Solution;

    fn ask(&self) -> Answer {
        let question_string: String = self.to_string();
        print!("    {}", question_string);
        std::io::stdout().flush().unwrap();

        let user_input = utils::get_user_input();
        let user_input = utils::remove_last_char_from_string(user_input);
        let answer: f64 = user_input.parse().unwrap();
        Answer(answer)
    }

    fn check_answer(&self, answer: &Answer) -> bool {
        let error = (self.solution().0 - answer.0).abs() / self.solution().0;
        let correct = error < 0.05;
        let correct_str = match correct {
            true => format!("true ").green(),
            false => format!("false").red(),
        };
        let text = format!(
            "\t\t\t{} {}{}",
            correct_str,
            self.to_string(),
            self.solution().0,
        );
        println!("{}", text);
        correct
    }
}
