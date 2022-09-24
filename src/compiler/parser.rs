use crate::automaton::nfa::NFA;

use super::{
    ast::{Interpreter, NewNode, NodeKind},
    token::{Token, TokenKind},
    Context,
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

    pub fn parse(&mut self) -> NFA {
        self.expr()
    }

    /// expr = sub_expr EOF
    pub fn expr(&mut self) -> NFA {
        let node = self.sub_expr();
        self.expect(TokenKind::Eof);

        let mut context = Context::default();
        let fragment = node.assemble(&mut context);
        let nfa = fragment.build();
        nfa
    }

    /// sub_expr = (seq '|' sub_expr) | seq
    pub fn sub_expr(&mut self) -> NewNode {
        let node = self.seq();

        if self.peek().kind == TokenKind::Union {
            self.expect(TokenKind::Union);
            let right = self.sub_expr();
            let new = self.new_union(Some(node), Some(right));
            new
        } else {
            node
        }
    }

    /// sequence = sub_sequence | ""
    pub fn seq(&mut self) -> NewNode {
        match self.peek().kind {
            TokenKind::LParen | TokenKind::Char => {
                let node = self.sub_seq();
                node
            }
            // FIXME: RPAREN が来るケースもあるようだが具体例を思いつかないのでそのテストケースを作った時に追加する
            TokenKind::Union | TokenKind::Eof => self.new_char("".to_string()),
            _ => panic!("unexpected token: {:?}", self.peek()),
        }
    }

    /// sub_sequence = star sub_sequence | star
    pub fn sub_seq(&mut self) -> NewNode {
        let node = self.star();

        match self.peek().kind {
            TokenKind::LParen | TokenKind::Char => {
                let right = self.sub_seq();
                self.new_concat(Some(node), Some(right))
            }
            TokenKind::Union | TokenKind::RParen | TokenKind::Eof => {
                return node;
            }
            _ => panic!("unexpected token: {:?}", self.peek()),
        }
    }

    /// star = primary | primary"*"
    pub fn star(&mut self) -> NewNode {
        let mut node = self.primary();
        if self.peek().kind == TokenKind::Star {
            self.expect(TokenKind::Star);
            node = self.new_star(Some(node));
        }
        node
    }

    /// primary = "(" sub_expr ")" | CHAR
    pub fn primary(&mut self) -> NewNode {
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

    pub fn new_union(&mut self, left: Option<NewNode>, right: Option<NewNode>) -> NewNode {
        NewNode::new(NodeKind::Union, None, left, right)
    }

    pub fn new_concat(&mut self, left: Option<NewNode>, right: Option<NewNode>) -> NewNode {
        NewNode::new(NodeKind::Concat, None, left, right)
    }

    pub fn new_star(&mut self, origin: Option<NewNode>) -> NewNode {
        NewNode::new(NodeKind::Star, None, origin, None)
    }

    pub fn new_char(&mut self, ch: String) -> NewNode {
        NewNode::new(NodeKind::Char, Some(ch), None, None)
    }
}
