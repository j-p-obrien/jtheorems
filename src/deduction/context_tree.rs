use std::{
    hint::unreachable_unchecked,
    ops::{Index, IndexMut},
};

use crate::term::{lambda::PiType, types::Type, variable::FreeVariable};

use super::{
    error::JError,
    judgement::{Judgement, JudgementType},
    term_arena::TermArena,
    terminal::JResult,
};

type ContextPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Empty {
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
pub(super) enum Context {
    Empty(Empty),
    NonEmpty(NonEmpty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ContextTree {
    nodes: Vec<Context>,
}

impl From<ContextPtrSize> for ContextPtr {
    fn from(index: ContextPtrSize) -> Self {
        Self(index)
    }
}

impl Index<ContextPtr> for ContextTree {
    type Output = Context;

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

    pub(super) fn is_empty_context(&self) -> bool {
        self.0 == 0
    }
}

impl Empty {
    fn new() -> Self {
        Self {
            constructed: vec![],
            reachable: vec![],
        }
    }
}

impl NonEmpty {
    fn new(variable: FreeVariable, parent: ContextPtr) -> Self {
        Self {
            variable,
            parent,
            constructed: vec![],
            reachable: vec![],
        }
    }

    pub(super) fn variable(&self) -> FreeVariable {
        self.variable
    }

    pub(super) fn parent(&self) -> ContextPtr {
        self.parent
    }
}

impl Context {
    fn empty_context() -> Self {
        Self::Empty(Empty::new())
    }

    fn new_context(variable: FreeVariable, parent: ContextPtr) -> Self {
        Self::NonEmpty(NonEmpty::new(variable, parent))
    }

    fn add_judgement(&mut self, judgement_type: JudgementType) {
        match self {
            Context::Empty(root) => root.constructed.push(judgement_type),
            Context::NonEmpty(node) => node.constructed.push(judgement_type),
        }
    }

    fn add_reachable_context(&mut self, context: ContextPtr) {
        match self {
            Context::Empty(root) => root.reachable.push(context),
            Context::NonEmpty(node) => node.reachable.push(context),
        }
    }
}

impl ContextTree {
    pub(super) fn new() -> Self {
        Self {
            nodes: vec![Context::empty_context()],
        }
    }

    pub(super) fn add_judgement_at(&mut self, judgement: JudgementType, context: ContextPtr) {
        self[context].add_judgement(judgement)
    }

    /// Sometimes we need to get the internal NonEmpty node of a ContextTreeNode after we've already
    /// checked that it is not the empty Context. This is a fast way of returning that node.
    ///
    /// This function should only be used if you have already checked that ContextPtr is
    /// not root and also is less than the length of the ContextTree (Which should hopefully always
    /// be the case anyways).
    pub(super) unsafe fn get_nonempty_unchecked(&self, context: ContextPtr) -> &NonEmpty {
        if let Context::NonEmpty(node) = &self[context] {
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
        context: ContextPtr,
        term_data: &TermArena,
    ) -> bool {
        let mut current = context;
        while !current.is_empty_context() {
            // SAFETY: We have already checked that current is not the Empty Context.
            let node = unsafe { self.get_nonempty_unchecked(current) };
            if node.variable.has_name(name, term_data) {
                return true;
            }
            current = node.parent;
        }
        false
    }

    /// Given the FreeVariable and the Context, this function creates a new Context with the
    /// FreeVariable, adds it to the reachable Contexts for the old Context, and returns the new
    /// Context.
    pub(super) fn context_extension_at(
        &mut self,
        variable: FreeVariable,
        context: ContextPtr,
    ) -> ContextPtr {
        let new_context: ContextPtr = self.nodes.len().into();
        let new_node = Context::new_context(variable, context);
        self.nodes.push(new_node);
        self[context].add_reachable_context(new_context);
        new_context
    }

    pub(super) fn try_variable_introduction_at(
        &mut self,
        name: &str,
        context: ContextPtr,
        term_data: &TermArena,
    ) -> JResult<FreeVariable> {
        let mut current = context;
        while !current.is_empty_context() {
            // SAFETY: We have already checked that current is not the Empty Context.
            let node = unsafe { self.get_nonempty_unchecked(current) };
            let variable = node.variable();
            if variable.has_name(name, term_data) {
                let judgement: JudgementType = variable.into();
                self.add_judgement_at(judgement, context);
                return Ok(variable);
            }
            current = node.parent;
        }
        Err(JError::Illegal("Variable not found in Context."))
    }

    pub(super) fn pi_formation_at(
        &mut self,
        typ: Type,
        context: ContextPtr,
        term_data: &mut TermArena,
    ) -> PiType {
        todo!("Implement pi_formation_at.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_tree() {}
}
