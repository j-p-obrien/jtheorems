use std::{
    hint::unreachable_unchecked,
    ops::{Index, IndexMut},
};

use crate::terms::variable::FreeVariable;

use super::{judgement::JudgementType, term_arena::TermArena};

type ContextPtr = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Context(ContextPtr);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EmptyContext {
    constructed: Vec<JudgementType>,
    reachable: Vec<Context>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NonEmpty {
    variable: FreeVariable,
    parent: Context,
    constructed: Vec<JudgementType>,
    reachable: Vec<Context>,
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

impl From<ContextPtr> for Context {
    fn from(index: ContextPtr) -> Self {
        Self(index)
    }
}

impl Index<Context> for ContextTree {
    type Output = ContextTreeNode;

    fn index(&self, index: Context) -> &Self::Output {
        if cfg!(debug_assertions) {
            &self.nodes[index.index()]
        } else {
            // SAFETY: The index should always be in bounds.
            unsafe { self.nodes.get_unchecked(index.index()) }
        }
    }
}

impl IndexMut<Context> for ContextTree {
    fn index_mut(&mut self, index: Context) -> &mut Self::Output {
        if cfg!(debug_assertions) {
            &mut self.nodes[index.index()]
        } else {
            // SAFETY: The index should always be in bounds.
            unsafe { self.nodes.get_unchecked_mut(index.index()) }
        }
    }
}

impl Context {
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

impl EmptyContext {
    fn new() -> Self {
        Self {
            constructed: vec![],
            reachable: vec![],
        }
    }
}

impl NonEmpty {
    fn new(variable: FreeVariable, parent: Context) -> Self {
        Self {
            variable,
            parent,
            constructed: vec![],
            reachable: vec![],
        }
    }
}

impl ContextTreeNode {
    fn empty_context() -> Self {
        Self::EmptyContext(EmptyContext::new())
    }

    fn new_context(variable: FreeVariable, parent: Context) -> Self {
        Self::NonEmpty(NonEmpty::new(variable, parent))
    }

    fn add_judgement(&mut self, judgement_type: JudgementType) {
        match self {
            ContextTreeNode::EmptyContext(root) => root.constructed.push(judgement_type),
            ContextTreeNode::NonEmpty(node) => node.constructed.push(judgement_type),
        }
    }

    fn add_reachable_context(&mut self, context: Context) {
        match self {
            ContextTreeNode::EmptyContext(root) => root.reachable.push(context),
            ContextTreeNode::NonEmpty(node) => node.reachable.push(context),
        }
    }
}

impl ContextTree {
    pub(super) fn new() -> Self {
        Self {
            nodes: vec![ContextTreeNode::empty_context()],
        }
    }

    pub(super) fn add_judgement_at(&mut self, judgement_type: JudgementType, context: Context) {
        self[context].add_judgement(judgement_type)
    }

    /// This function should only be used if you have already checked that ContextPtr is
    /// not root and also is less than the length of the ContextTree (Which should hopefully always
    /// be the case anyways).
    unsafe fn get_child_node_unchecked(&self, context: Context) -> &NonEmpty {
        if let ContextTreeNode::NonEmpty(node) = &self[context] {
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
        term_data: &TermArena,
        context: Context,
    ) -> bool {
        let mut current = context;
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

    /// Given the FreeVariable and the Context, this function creates a new Context with the
    /// FreeVariable, adds it to the reachable Contexts for the old Context, and returns the new
    /// Context.
    pub(super) fn variable_introduction_at(
        &mut self,
        variable: FreeVariable,
        context: Context,
    ) -> Context {
        let new_context: Context = self.nodes.len().into();
        let new_node = ContextTreeNode::new_context(variable, context);
        self.nodes.push(new_node);
        self[context].add_reachable_context(new_context);
        new_context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_tree() {}
}
