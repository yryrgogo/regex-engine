#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    CHAR = 0,
    OP_UNION = 1,
    OP_STAR = 2,
    LPAREN = 3,
    RPAREN = 4,
    EOF = 5,
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
