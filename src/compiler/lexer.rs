use super::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    fn next(&mut self) -> Token {
        let c = self.input.chars().nth(self.pos);
        self.pos += 1;

        if let Some(ch) = c {
            match ch {
                'a'..='z' => Token::new(TokenKind::Char, Some(ch.to_string())),
                '|' => Token::new(TokenKind::Union, None),
                '*' => Token::new(TokenKind::Star, None),
                '(' => Token::new(TokenKind::LParen, None),
                ')' => Token::new(TokenKind::RParen, None),
                _ => panic!("invalid character: {}", ch),
            }
        } else {
            Token::new(TokenKind::Eof, None)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let token = self.next();
            let kind = token.kind.clone();
            tokens.push(token);
            if kind == TokenKind::Eof {
                break;
            }
        }
        tokens
    }
}
