use std::{
    fmt::Display,
};

use crate::terms::{
    variable::{FreeVariable, VariableData},
    Term, TermData,
};

type PreviousIdx = usize;

#[derive(Debug)]
/// Tree that stores the Context.
/// 
/// FreeVariable is a Variable that has been legally introduced into the Context.
/// PreviousIdx is the index of the previous FreeVariable in the ContextTree. Since the first 
/// FreeVariablevin a context does not have a predecessor, this value is optional.
struct ContextTree {
    tree: Vec<(FreeVariable, Option<PreviousIdx>)>,
}


#[derive(Debug)]
/// The Context
/// 
/// Contains the ContextTree and the associated data for each FreeVariable in the Context Tree
pub(crate) struct Context {
    tree: ContextTree,
    variables: Vec<VariableData>,
}

impl ContextTree {
    fn new() -> Self {
        Self {
            tree: vec![]
        }
    }
}


impl Context {
    pub(crate) fn new() -> Self {
        Self { tree: ContextTree::new(), variables: vec![] }
    }

    pub(crate) fn push(&mut self, variable: FreeVariable) {
        todo!()
    }

    pub(crate) fn name_is_taken(&self, name: &str) -> bool {
        todo!()
    }
}
