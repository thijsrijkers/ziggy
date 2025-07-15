#[derive(Debug)]
#[allow(dead_code)]
pub enum Token {
    Number(String),
    Identifier(String),
    Operator(char),
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    BlankLine,
    Unknown,
}

