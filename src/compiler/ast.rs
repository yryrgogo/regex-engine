#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Char,
    Union,
    Concat,
    Star,
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub ch: Option<String>,
}

impl Node {
    pub fn new(
        kind: NodeKind,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        ch: Option<String>,
    ) -> Self {
        Self {
            kind,
            left,
            right,
            ch,
        }
    }
}
