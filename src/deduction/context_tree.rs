use std::{
    hint::unreachable_unchecked,
    ops::{Index, IndexMut},
};

use crate::terms::{types::Type, variable::FreeVariable, Term};

use super::{
    judgement::{Judgement, JudgementType},
    term_arena::TermArena,
};

type ContextPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EmptyContext {
    constructed: Vec<JudgementType>,
    reachable: Vec<ContextPtr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NonEmpty {
    variable: FreeVariable,
    parent: ContextPtr,
    constructed: Vec<JudgementType>,
    reachable: Vec<ContextPtr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ContextTreeNode {
    EmptyContext(EmptyContext),
    NonEmpty(NonEmpty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTree {
    nodes: Vec<ContextTreeNode>,
}

impl Index<ContextPtr> for ContextTree {
    type Output = ContextTreeNode;

    fn index(&self, index: ContextPtr) -> &Self::Output {
        if cfg!(debug_assertions) {
            &self.nodes[index.index()]
        } else {
            // SAFETY: The index should always be in bounds.
            unsafe { self.nodes.get_unchecked(index.index()) }
        }
    }
}

impl IndexMut<ContextPtr> for ContextTree {
    fn index_mut(&mut self, index: ContextPtr) -> &mut Self::Output {
        if cfg!(debug_assertions) {
            &mut self.nodes[index.index()]
        } else {
            // SAFETY: The index should always be in bounds.
            unsafe { self.nodes.get_unchecked_mut(index.index()) }
        }
    }
}

impl ContextPtr {
    /// Creates a ContextPtr that points to the root of the ContextTree i.e. at index 0.
    pub(super) fn empty_context() -> Self {
        Self(0)
    }

    fn index(&self) -> usize {
        self.0
    }

    fn is_empty_context(&self) -> bool {
        self.0 == 0
    }
}

impl ContextTreeNode {
    fn empty_context() -> Self {
        Self::EmptyContext(EmptyContext {
            constructed: vec![],
            reachable: vec![],
        })
    }

    fn push(&mut self, judgement_type: JudgementType) {
        match self {
            ContextTreeNode::EmptyContext(root) => root.constructed.push(judgement_type),
            ContextTreeNode::NonEmpty(node) => node.constructed.push(judgement_type),
        }
    }
}

impl ContextTree {
    pub(super) fn new() -> Self {
        Self {
            nodes: vec![ContextTreeNode::empty_context()],
        }
    }

    pub(super) fn push_at(&mut self, location: ContextPtr, judgement_type: JudgementType) {
        self[location].push(judgement_type)
    }

    /// This function should only be used if you have already checked that ContextPtr is
    /// not root and also is less than the length of the ContextTree (Which should hopefully always
    /// be the case anyways).
    unsafe fn get_child_node_unchecked(&self, location: ContextPtr) -> &NonEmpty {
        if let ContextTreeNode::NonEmpty(node) = &self[location] {
            node
        } else if cfg!(debug_assertions) {
            unreachable!("ContextPtr should not be root when this function is called.")
        } else {
            // SAFETY: ContextPtr should not be root when this function is called.
            unsafe { unreachable_unchecked() }
        }
    }

    pub(super) fn contains_name_at(
        &self,
        name: &str,
        location: ContextPtr,
        term_data: &TermArena,
    ) -> bool {
        let mut current = location;
        loop {
            if current.is_empty_context() {
                return false;
            }
            // SAFETY: We have already checked that current is not root.
            let node = unsafe { self.get_child_node_unchecked(current) };
            if node.variable.has_name(name, term_data) {
                return true;
            }
            current = node.parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_variable_x() {
        let mut term_data = TermArena::new();
        let mut context_tree = ContextTree::new();
        todo!()
    }

    #[test]
    fn test_context_tree() {}
}
