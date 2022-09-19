#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Char,
    Union,
    Star,
    LParen,
    RParen,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, val: Option<String>) -> Self {
        Self { kind, val }
    }
}
