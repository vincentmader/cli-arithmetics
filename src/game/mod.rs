mod game_mode;
use crate::answer::Answer;
use crate::question::Question;
use crate::question::QuestionV01;
use crate::question::QuestionV02;
use crate::question::QuestionV03;
use crate::question::QuestionV04;
use crate::question::QuestionV05;
use crate::utils;
pub use game_mode::GameMode;
use std::time::SystemTime;

pub struct Game {
    mode: GameMode,
}
impl Game {
    pub fn new() -> Self {
        let mode = utils::ask_for_gamemode();
        Game { mode }
    }

    pub fn init(&mut self) {
        loop {
            let question: Box<dyn Question> = match &self.mode {
                GameMode::GameModeV01 => Box::new(QuestionV01::new()),
                GameMode::GameModeV02 => Box::new(QuestionV02::new()),
                GameMode::GameModeV03 => Box::new(QuestionV03::new()),
                GameMode::GameModeV04 => Box::new(QuestionV04::new()),
                GameMode::GameModeV05 => Box::new(QuestionV05::new()),
            };

            let time = SystemTime::now();
            let answer = question.ask();
            let duration = time.elapsed().unwrap().as_millis();

            let timestamp = time
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let _correct = question.check_answer(&answer);

            self.save_question_to_file(timestamp, question, answer, duration);
        }
    }

    pub fn save_question_to_file(
        &self,
        timestamp: u128,
        question: Box<dyn Question>,
        answer: Answer,
        duration: u128,
    ) {
        let path_to_file = question.variant().into_saves_path();
        let new_content = format!(
            "{:?};{};{};{};{}\n",
            timestamp,
            question.to_string(),
            question.solution().0,
            answer.0,
            duration
        );
        utils::append_to_file(&path_to_file, &new_content);
    }
}
