use crate::{
    deduction::{
        judgement::{ContextPtr, Judgement, JudgementKind},
        judgement_tree::JudgementTree,
    },
    terms::{
        variable::{FreeVariable, VariableData},
        TermData, Type,
    },
};

type ParentPtr = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ParentPtr>,
    judgement_tree: JudgementTree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContextTree {
    nodes: Vec<ContextTreeNode>,
}

impl ContextTreeNode {
    fn root() -> Self {
        Self {
            free_variable: None,
            parent: None,
            judgement_tree: JudgementTree::new(),
        }
    }
}

impl ContextTree {
    pub(crate) fn new() -> Self {
        Self {
            nodes: vec![ContextTreeNode::root()],
        }
    }
}
