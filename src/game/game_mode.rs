pub enum GameMode {
    GameModeV01,
    GameModeV02,
    GameModeV03,
    GameModeV04,
    GameModeV05,
}
impl GameMode {
    pub fn into_saves_path(&self) -> String {
        match self {
            Self::GameModeV01 => "saves/gamemode_v01.txt",
            Self::GameModeV02 => "saves/gamemode_v02.txt",
            Self::GameModeV03 => "saves/gamemode_v03.txt",
            Self::GameModeV04 => "saves/gamemode_v04.txt",
            Self::GameModeV05 => "saves/gamemode_v05.txt",
        }
        .to_owned()
    }
}
impl ToString for GameMode {
    fn to_string(&self) -> String {
        match self {
            Self::GameModeV01 => "Find x^2.",
            Self::GameModeV02 => "Find x*y.",
            Self::GameModeV03 => "Find x~y.",
            Self::GameModeV04 => "Find 2^x.",
            Self::GameModeV05 => "Find x^y.",
        }
        .to_owned()
    }
}
