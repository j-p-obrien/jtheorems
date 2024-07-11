use crate::terms::variable::{FreeVariable, VariableData};

type ParentIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTreeNode {
    free_variable: FreeVariable,
    parent: Option<ParentIdx>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
/// Tree that stores the Context.
/// 
/// FreeVariable is a Variable that has been legally introduced into the Context.
/// PreviousIdx is the index of the previous FreeVariable in the ContextTree. Since the first 
/// FreeVariable in a context does not have a predecessor, this value is optional.
pub(crate) struct ContextTree(Vec<ContextTreeNode>);


impl ContextTree {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }
}