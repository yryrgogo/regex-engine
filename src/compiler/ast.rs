use std::collections::HashSet;

use super::{
    fragment::{NFAFragment, NFAInput},
    Context,
};

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

#[derive(Debug, Clone)]
pub enum NewNode {
    Char(CharacterNode),
    Union(UnionNode),
}

impl Interpreter for NewNode {
    fn assemble(&self, ctx: &mut Context) -> NFAFragment {
        match self {
            NewNode::Char(node) => node.assemble(ctx),
            NewNode::Union(node) => node.assemble(ctx),
        }
    }
}

trait Interpreter {
    fn assemble(&self, context: &mut Context) -> NFAFragment;
}

#[derive(Debug, Clone)]
pub struct CharacterNode {
    pub ch: String,
}

impl Interpreter for CharacterNode {
    fn assemble(&self, context: &mut Context) -> NFAFragment {
        let start = context.new_state();
        let accept = context.new_state();

        let mut fragment = NFAFragment::new(start, HashSet::from_iter(vec![accept].into_iter()));
        fragment.connect(NFAInput::new(self.ch.clone(), start), accept);
        fragment
    }
}

#[derive(Debug, Clone)]
pub struct UnionNode {
    pub left: Box<NewNode>,
    pub right: Box<NewNode>,
}

impl Interpreter for UnionNode {
    fn assemble(&self, context: &mut Context) -> NFAFragment {
        let left = self.left.assemble(context);
        let right = self.right.assemble(context);

        let start = context.new_state();
        let mut accepts = left
            .accepts
            .unwrap_or_else(|| panic!("left.accepts is None"))
            .clone();
        accepts.extend(
            &right
                .accepts
                .unwrap_or_else(|| panic!("right.accepts is None")),
        );

        let mut fragment = NFAFragment::new(start, accepts);
        fragment.connect(
            NFAInput::new("".to_string(), start),
            left.start.unwrap_or_else(|| panic!("left.start is None")),
        );
        fragment.connect(
            NFAInput::new("".to_string(), start),
            right.start.unwrap_or_else(|| panic!("right.start is None")),
        );
        fragment
    }
}

#[derive(Debug, Clone)]
pub struct ConcatNode {
    pub left: Box<NewNode>,
    pub right: Box<NewNode>,
}

impl Interpreter for ConcatNode {
    fn assemble(&self, context: &mut Context) -> NFAFragment {
        let left = self.left.assemble(context);
        let right = self.right.assemble(context);

        let mut fragment = NFAFragment::new(
            left.start.unwrap_or_else(|| panic!("left.start is None")),
            right
                .accepts
                .unwrap_or_else(|| panic!("right.accepts is None")),
        );

        for state in left
            .accepts
            .unwrap_or_else(|| panic!("left.accepts is None"))
            .clone()
        {
            fragment.connect(
                NFAInput::new("".to_string(), state),
                right.start.unwrap_or_else(|| panic!("right.start is None")),
            );
        }
        fragment
    }
}

#[derive(Debug, Clone)]
pub struct StarNode {
    pub origin: Box<NewNode>,
}

impl Interpreter for StarNode {
    fn assemble(&self, context: &mut Context) -> NFAFragment {
        let origin = self.origin.assemble(context);

        let start = context.new_state();
        let mut accepts = origin
            .accepts
            .clone()
            .unwrap_or_else(|| panic!("origin.accepts is None"));
        accepts.insert(start);

        let mut fragment = origin.new_skeleton();
        fragment.start = Some(start);
        fragment.accepts = Some(accepts);

        for state in origin
            .accepts
            .unwrap_or_else(|| panic!("left.accepts is None"))
        {
            fragment.connect(
                NFAInput::new("".to_string(), state),
                origin
                    .start
                    .unwrap_or_else(|| panic!("origin.start is None")),
            );
        }
        fragment.connect(
            NFAInput::new("".to_string(), start),
            origin
                .start
                .unwrap_or_else(|| panic!("origin.start is None")),
        );

        fragment
    }
}
