use crate::terms::{types::Type, variable::FreeVariable};

use super::{context_tree::ContextTree, judgement::ContextPtr, term_arena::TermArena};

/// The Domain is the central repository of all Terms and their associated Context. This data
/// structure is the heart of the proof system.
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
