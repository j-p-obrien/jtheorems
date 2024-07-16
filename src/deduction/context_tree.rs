use std::ops::{Index, IndexMut};

use crate::terms::{types::Type, variable::FreeVariable, Term};

use super::judgement::{Judgement, JudgementType};

type ContextPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ContextPtr>,
    constructible: Vec<JudgementType>,
    reachable: Vec<ContextPtr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTree {
    nodes: Vec<ContextTreeNode>,
}

impl Index<ContextPtr> for ContextTree {
    type Output = ContextTreeNode;

    fn index(&self, index: ContextPtr) -> &Self::Output {
        &self.nodes[index.index()]
    }
}

impl IndexMut<ContextPtr> for ContextTree {
    fn index_mut(&mut self, index: ContextPtr) -> &mut Self::Output {
        &mut self.nodes[index.index()]
    }
}

impl ContextPtr {
    /// Creates a ContextPtr that points to the root of the ContextTree i.e. at index 0.
    pub(super) fn empty_context() -> Self {
        Self(0)
    }

    pub(super) fn index(&self) -> usize {
        self.0
    }
}

impl ContextTreeNode {
    fn root() -> Self {
        Self {
            free_variable: None,
            parent: None,
            constructible: vec![JudgementType::well_formed()],
            reachable: vec![],
        }
    }

    pub(super) fn push(&mut self, judgement_type: JudgementType) {
        self.constructible.push(judgement_type)
    }
}

impl ContextTree {
    pub(super) fn root() -> Self {
        Self {
            nodes: vec![ContextTreeNode::root()],
        }
    }
}
