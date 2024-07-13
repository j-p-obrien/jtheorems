use crate::{deduction::judgement_tree::JudgementTree, terms::variable::FreeVariable};

type ParentPtr = usize;
type ContextPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ParentPtr>,
    judgement_tree: JudgementTree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTree {
    nodes: Vec<ContextTreeNode>,
}

impl ContextPtr {
    /// Creates a ContextPtr that points to the root of the ContextTree i.e. at index 0.
    pub(super) fn empty_context() -> Self {
        Self(0)
    }
}

impl ContextTreeNode {
    fn root() -> Self {
        Self {
            free_variable: None,
            parent: None,
            judgement_tree: JudgementTree::root(),
        }
    }
}

impl ContextTree {
    pub(super) fn root() -> Self {
        Self {
            nodes: vec![ContextTreeNode::root()],
        }
    }
}
