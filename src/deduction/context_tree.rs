use std::ops::{Index, IndexMut};

use crate::terms::{types::Type, variable::FreeVariable, Term};

use super::{
    judgement::{Judgement, JudgementType},
    term_arena::TermArena,
};

type ContextPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ContextPtr>,
    constructed: Vec<JudgementType>,
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
            constructed: vec![],
            reachable: vec![],
        }
    }

    pub(super) fn push(&mut self, judgement_type: JudgementType) {
        self.constructed.push(judgement_type)
    }
}

impl ContextTree {
    pub(super) fn root() -> Self {
        Self {
            nodes: vec![ContextTreeNode::root()],
        }
    }

    pub(super) fn contains_name_at(
        &self,
        name: &str,
        location: ContextPtr,
        term_data: &TermArena,
    ) -> bool {
        if let Some(variable) = self[location].free_variable {
            if variable.data.name() == name {
                return true;
            }
        }
        let mut current = location;
        while let Some(parent) = self[current].parent {
            if let Some(free_variable) = self[parent].free_variable {
                if free_variable.data.name() == name {
                    return true;
                }
            }
            current = parent;
        }
        false
    }
}
