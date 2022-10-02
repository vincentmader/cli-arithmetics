pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::Addition => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
        }
        .to_owned()
    }
}
