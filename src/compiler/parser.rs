use super::{
    ast::Node,
    token::{Token, TokenKind},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expect(&self, kind: TokenKind) {
        if self.tokens[self.current].kind == kind {
            self.current += 1;
        }
        panic!(
            "expected {:?}, got {:?}",
            kind, self.tokens[self.current].kind
        );
    }

    /// primary = "(" expr ")" | CHAR
    pub fn primary(&mut self) -> Node {
        let token = self
            .tokens
            .get(self.current)
            .unwrap_or_else(|| panic!("unexpected index: {}", self.current));
        self.current += 1;

        match token.kind {
            super::token::TokenKind::CHAR => Node::new(token),
            super::token::TokenKind::LPAREN => {
                let node = self.expr();
                self.expect(TokenKind::RPAREN);
                node
            }
            _ => unreachable!(),
        }
    }

    /// start = primary | primary"*"
    pub fn star(&mut self) -> Node {
        let mut node = self.primary();
        while self
            .tokens
            .get(self.current)
            .map(|t| t.kind == super::token::TokenKind::OP_STAR)
            .unwrap_or(false)
        {
            self.current += 1;
            node = Node::new(&self.tokens[self.current - 1]);
            node.left = Some(Box::new(node));
        }
        node
    }
}
