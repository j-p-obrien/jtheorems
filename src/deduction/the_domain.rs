use crate::terms::{variable::FreeVariable, Type};

use super::{context_tree::ContextTree, judgement::ContextPtr, term_arena::TermArena};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TheDomain {
    context_tree: ContextTree,
    term_data: TermArena,
}

impl TheDomain {
    pub(crate) fn new() -> Self {
        Self {
            context_tree: ContextTree::new(),
            term_data: TermArena::new(),
        }
    }

    pub(crate) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        todo!()
    }

    pub(crate) fn push_variable(&mut self, name: String, typ: Type) -> FreeVariable {
        todo!()
    }
}
