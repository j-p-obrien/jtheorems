use crate::{
    deduction::judgement::ContextPtr,
    terms::variable::{FreeVariable, VariableData},
};

type ParentPtr = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTreeNode {
    free_variable: FreeVariable,
    parent: ParentPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tree that stores the Context.
///
/// FreeVariable is a Variable that has been legally introduced into the Context.
/// PreviousIdx is the index of the previous FreeVariable in the ContextTree. Since the first
/// FreeVariable in a context does not have a predecessor, this value is optional.
pub(crate) struct ContextTree(Vec<Option<ContextTreeNode>>);

impl ContextTree {
    pub(crate) fn new() -> Self {
        Self(vec![None])
    }

    pub(crate) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        todo!()
    }

    pub(crate) fn push_variable(&mut self, variable_data: VariableData) -> FreeVariable {
        todo!()
    }
}
