use super::{ast::Node, token::Token};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn primary(&self) -> Node {
        let token = self
            .tokens
            .get(self.current)
            .unwrap_or_else(|| panic!("unexpected index: {}", self.current));

        Node::new(token)
    }
}
