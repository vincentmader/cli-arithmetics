use crate::game::GameMode;
use colored::Colorize;
use std::io::Write;

pub fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input
}

pub fn remove_last_char_from_string(x: String) -> String {
    let mut chars = x.chars();
    chars.next_back();
    chars.as_str().to_owned()
}

pub fn append_to_file(path_to_file: &str, new_content: &str) {
    let content = std::fs::read_to_string(path_to_file).unwrap();
    let new_content = format!("{}{}", content, new_content);
    std::fs::write(path_to_file, new_content).unwrap();
}

pub fn ask_for_gamemode() -> GameMode {
    println!("{}", format!("\n  Choose game-mode:").blue());
    println!("{}", format!("  -----------------").blue());
    let lines = [
        "Find x^2 for x   in [0, 25].",
        "Find x*y for x,y in [0, 25].",
        "Find x~y for x,y in [0, 25], & for ~ in {{+,-,*,/}}.",
        "Find 2^x for x   in [0, 10].",
        "Find x^y for x   in [0, 10], & for y in [0, 5].",
    ];
    for (line_idx, line) in lines.iter().enumerate() {
        let line_idx = format!("{}", line_idx + 1).blue();
        println!("    {}. {}", line_idx, line);
    }
    print!("{}", format!("\n  Choice: ").blue());
    std::io::stdout().flush().unwrap();

    let game_mode = get_user_input();
    let game_mode = remove_last_char_from_string(game_mode);
    let game_mode = match game_mode.as_str() {
        "1" => GameMode::GameModeV01,
        "2" => GameMode::GameModeV02,
        "3" => GameMode::GameModeV03,
        "4" => GameMode::GameModeV04,
        "5" => GameMode::GameModeV05,
        _ => todo!("Implement other game-modes."),
    };
    println!("  -> {}\n", game_mode.to_string());
    game_mode
}
