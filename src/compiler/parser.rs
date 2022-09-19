use super::{
    ast::{Node, NodeKind},
    token::{Token, TokenKind},
};

/// parse a list of tokens into an AST
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Node {
        self.expr()
    }

    pub fn next_token(&mut self) -> &Token {
        let token = self
            .tokens
            .get(self.current)
            .unwrap_or_else(|| panic!("unexpected index: {}/{}", self.current, self.tokens.len()));
        self.current += 1;
        token
    }

    pub fn peek(&mut self) -> &Token {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| panic!("unexpected index: {}/{}", self.current, self.tokens.len()))
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.tokens[self.current].kind == kind {
            self.current += 1;
            return;
        } else {
            panic!(
                "expected {:?}, got {:?}",
                kind, self.tokens[self.current].kind
            );
        }
    }

    pub fn new_node(&mut self, kind: NodeKind, left: Option<Node>, right: Option<Node>) -> Node {
        if kind == NodeKind::Char {
            panic!("new_node with CHAR is not allowed");
        }

        match (left, right) {
            (None, None) => Node::new(kind, None, None, None),
            (None, Some(_)) => unreachable!(),
            (Some(left), None) => Node::new(kind, Some(Box::new(left)), None, None),
            (Some(left), Some(right)) => {
                Node::new(kind, Some(Box::new(left)), Some(Box::new(right)), None)
            }
        }
    }

    pub fn new_char(&mut self, ch: String) -> Node {
        Node::new(NodeKind::Char, None, None, Some(ch))
    }

    /// expr = sub_expr EOF
    pub fn expr(&mut self) -> Node {
        let node = self.sub_expr();
        self.expect(TokenKind::Eof);
        node
    }

    /// sub_expr = (seq '|' sub_expr) | seq
    pub fn sub_expr(&mut self) -> Node {
        let node = self.seq();

        if self.peek().kind == TokenKind::Union {
            self.expect(TokenKind::Union);
            let right = self.sub_expr();
            self.new_node(NodeKind::Union, Some(node), Some(right))
        } else {
            node
        }
    }

    /// sequence = sub_sequence | ""
    pub fn seq(&mut self) -> Node {
        match self.peek().kind {
            TokenKind::LParen | TokenKind::Char => {
                let node = self.sub_seq();
                node
            }
            // FIXME: RPAREN が来るケースもあるようだが具体例を思いつかないのでそのテストケースを作った時に追加する
            TokenKind::Union | TokenKind::Eof => {
                Node::new(NodeKind::Char, None, None, Some("".to_string()))
            }
            _ => panic!("unexpected token: {:?}", self.peek()),
        }
    }

    /// sub_sequence = star sub_sequence | star
    pub fn sub_seq(&mut self) -> Node {
        let node = self.star();

        match self.peek().kind {
            TokenKind::LParen | TokenKind::Char => {
                let right = self.sub_seq();
                self.new_node(NodeKind::Concat, Some(node), Some(right))
            }
            TokenKind::Union | TokenKind::RParen | TokenKind::Eof => {
                return node;
            }
            _ => panic!("unexpected token: {:?}", self.peek()),
        }
    }

    /// star = primary | primary"*"
    pub fn star(&mut self) -> Node {
        let mut node = self.primary();
        if self.peek().kind == TokenKind::Star {
            self.expect(TokenKind::Star);
            node = self.new_node(NodeKind::Star, Some(node), None);
        }
        node
    }

    /// primary = "(" sub_expr ")" | CHAR
    pub fn primary(&mut self) -> Node {
        let token = self.next_token();
        let ch = token.val.clone();

        match token.kind {
            super::token::TokenKind::Char => self.new_char(ch.unwrap()),
            super::token::TokenKind::LParen => {
                let node = self.sub_expr();
                self.expect(TokenKind::RParen);
                node
            }
            _ => unreachable!(),
        }
    }
}
