use crate::terms::variable::{FreeVariable, VariableData};

type PreviousIdx = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tree that stores the Context.
/// 
/// FreeVariable is a Variable that has been legally introduced into the Context.
/// PreviousIdx is the index of the previous FreeVariable in the ContextTree. Since the first 
/// FreeVariable in a context does not have a predecessor, this value is optional.
pub(crate) struct ContextTree(Vec<(FreeVariable, Option<PreviousIdx>)>);


#[derive(Debug, Clone, PartialEq, Eq)]
/// The Context
/// 
/// Contains the ContextTree and the associated data for each FreeVariable in the Context Tree
pub(crate) struct Context {
    tree: ContextTree,
    variables: Vec<VariableData>,
}

impl ContextTree {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }
}


impl Context {
    pub(crate) fn new() -> Self {
        Self { tree: ContextTree::new(), variables: vec![] }
    }
}
