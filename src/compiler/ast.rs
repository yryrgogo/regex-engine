use super::token::Token;

pub struct Node<'a> {
    token: &'a Token,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn new(token: &'a Token) -> Self {
        Self {
            token,
            left: None,
            right: None,
        }
    }
}
