use super::token::{Token, TokenKind};

struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::new(TokenKind::EOF, None);
        }

        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;

        match c {
            'a'..='z' => Token::new(TokenKind::CHAR, Some(c.to_string())),
            '|' => Token::new(TokenKind::OP_UNION, None),
            '*' => Token::new(TokenKind::OP_STAR, None),
            '(' => Token::new(TokenKind::LPAREN, None),
            ')' => Token::new(TokenKind::RPAREN, None),
            _ => panic!("invalid character: {}", c),
        }
    }
}
