use std::collections::HashSet;

use super::{
    fragment::{NFAFragment, NFAInput},
    Context,
};

pub trait Interpreter {
    fn assemble(&self, context: &mut Context) -> NFAFragment;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Char,
    Union,
    Concat,
    Star,
}

#[derive(Debug, Clone)]
pub enum NewNode {
    Char(CharacterNode),
    Union(UnionNode),
    Concat(ConcatNode),
    Star(StarNode),
}

impl Interpreter for NewNode {
    fn assemble(&self, ctx: &mut Context) -> NFAFragment {
        match self {
            NewNode::Char(node) => node.assemble(ctx),
            NewNode::Union(node) => node.assemble(ctx),
            NewNode::Concat(node) => node.assemble(ctx),
            NewNode::Star(node) => node.assemble(ctx),
        }
    }
}

impl NewNode {
    pub fn new(
        kind: NodeKind,
        ch: Option<String>,
        left: Option<NewNode>,
        right: Option<NewNode>,
    ) -> Self {
        match kind {
            NodeKind::Char => Self::Char(CharacterNode {
                ch: ch.unwrap_or_else(|| panic!("ch is required for NodeKind::Char")),
            }),
            NodeKind::Union => Self::Union(UnionNode {
                left: Box::new(
                    left.unwrap_or_else(|| panic!("left NewNode is required for NodeKind::Union")),
                ),
                right: Box::new(
                    right
                        .unwrap_or_else(|| panic!("right NewNode is required for NodeKind::Union")),
                ),
            }),
            NodeKind::Concat => {
                Self::Concat(ConcatNode {
                    left: Box::new(left.unwrap_or_else(|| {
                        panic!("left NewNode is required for NodeKind::Concat")
                    })),
                    right: Box::new(right.unwrap_or_else(|| {
                        panic!("right NewNode is required for NodeKind::Concat")
                    })),
                })
            }
            NodeKind::Star => Self::Star(StarNode {
                origin: Box::new(
                    left.unwrap_or_else(|| panic!("right NewNode is required for NodeKind::Star")),
                ),
            }),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterNode {
    pub ch: String,
}

impl Interpreter for CharacterNode {
    fn assemble(&self, context: &mut Context) -> NFAFragment {
        let start = context.new_state();
        let accept = context.new_state();

        let mut fragment =
            NFAFragment::new(start, HashSet::from_iter(vec![accept].into_iter()), None);
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

        let mut accepts = left
            .accepts
            .unwrap_or_else(|| panic!("left.accepts is None"))
            .clone();
        accepts.extend(
            &right
                .accepts
                .unwrap_or_else(|| panic!("right.accepts is None")),
        );

        let start = context.new_state();
        let mut fragment = NFAFragment::new(start, accepts, None);
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

        let mut map = left.map.clone();
        map.extend(right.map);
        let mut fragment = NFAFragment::new(
            left.start.unwrap_or_else(|| panic!("left.start is None")),
            right
                .accepts
                .unwrap_or_else(|| panic!("right.accepts is None")),
            Some(map),
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

#[cfg(test)]
mod ast_tests {
    use std::collections::HashMap;

    use crate::automaton::{State, StateSet};

    use super::*;

    #[test]
    fn concat_assemble() {
        let node = ConcatNode {
            left: Box::new(NewNode::Char(CharacterNode {
                ch: "a".to_string(),
            })),
            right: Box::new(NewNode::Char(CharacterNode {
                ch: "b".to_string(),
            })),
        };

        let mut ctx = Context::default();
        let fragment = node.assemble(&mut ctx);

        let mut map: HashMap<NFAInput, StateSet> = HashMap::new();

        let mut accepts = HashSet::<State>::new();
        let mut ss0 = HashSet::<State>::new();
        let mut ss1 = HashSet::<State>::new();

        let left_start = State::new(0);
        let left_accept = State::new(1);
        let right_start = State::new(2);
        let right_accept = State::new(3);
        ss0.insert(left_accept);
        ss1.insert(right_start);

        accepts.insert(right_accept);

        map.insert(NFAInput::new("a".to_string(), left_start), ss0);
        map.insert(NFAInput::new("".to_string(), left_accept), ss1);
        map.insert(NFAInput::new("b".to_string(), right_start), accepts.clone());

        assert_eq!(
            fragment,
            NFAFragment {
                start: Some(left_start),
                accepts: Some(accepts),
                map,
            }
        );
    }
}
