use colored::Colorize;
use rs_math_game::game::GameMode;

const GAME_MODES: [GameMode; 5] = [
    GameMode::GameModeV01,
    GameMode::GameModeV02,
    GameMode::GameModeV03,
    GameMode::GameModeV04,
    GameMode::GameModeV05,
];

fn load_savefile_to_string(game_mode: &GameMode) -> String {
    let path_to_save = game_mode.into_saves_path();
    std::fs::read_to_string(path_to_save).unwrap()
}

fn main() {
    for (game_mode_id, game_mode) in GAME_MODES.iter().enumerate() {
        let game_mode_id = game_mode_id + 1;
        let game_mode_title = game_mode.to_string();

        let contents = load_savefile_to_string(game_mode);
        let lines: Vec<&str> = contents.split("\n").collect();
        let nr_of_lines = lines.len();

        let mut nr_of_correct_answers = 0;
        let mut nr_of_incorrect_answers = 0;
        let mut average_duration = 0;
        let mut variance = 0.;
        for line in lines[0..nr_of_lines - 1].iter() {
            let split: Vec<&str> = line.split(";").collect();
            let _time = split[0];
            let _question = split[1];

            let solution = split[2];
            let answer = split[3];
            if answer == solution {
                nr_of_correct_answers += 1
            } else {
                nr_of_incorrect_answers += 1
            }

            let duration = split[4];
            let duration: i32 = duration.parse().unwrap();
            average_duration += duration;

            let solution: f64 = solution.parse().unwrap();
            let answer: f64 = answer.parse().unwrap();
            if !(solution.is_nan() || answer.is_nan()) && solution.is_finite() && answer.is_finite()
            {
                variance += (solution - answer).powf(2.);
            }
        }
        let average_duration = average_duration as f64 / nr_of_lines as f64 / 1000.;
        let variance = variance as f64 / nr_of_lines as f64;
        let standard_deviation = variance.powf(0.5);

        let accuracy = nr_of_correct_answers as f64 / nr_of_lines as f64;

        let header_text = format!("\n  Game-Mode {}: {}", game_mode_id, game_mode_title).blue();
        println!("{}", header_text);
        println!("    - Questions answered:     {}", nr_of_lines);
        println!("    - Answered correctly:     {}", nr_of_correct_answers);
        println!("    - Answered incorrectly:   {}", nr_of_incorrect_answers);
        println!("    - Accuracy:               {:.1}%", accuracy * 100.);
        println!("    - Std. deviation:         {:.2}", standard_deviation);
        println!("    - Avg. answer time:       {:.1}s", average_duration);
    }
}
