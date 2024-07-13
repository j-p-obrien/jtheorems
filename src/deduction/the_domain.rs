use std::ops::{Index, IndexMut};

use crate::terms::{types::Type, variable::FreeVariable};

use super::{
    context_tree::{ContextPtr, ContextTree},
    judgement::{JudgementLocation, JudgementType},
    judgement_tree::{JudgementTree, JudgementTreeNode},
    term_arena::TermArena,
};

/// The Domain is the central repository of all Terms and their associated Context. This data
/// structure is the heart of the proof system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TheDomain {
    context_tree: ContextTree,
    term_data: TermArena,
}

impl Index<JudgementLocation> for TheDomain {
    type Output = JudgementTreeNode;

    fn index(&self, index: JudgementLocation) -> &Self::Output {
        todo!("Index for The Domain.")
    }
}

impl IndexMut<JudgementLocation> for TheDomain {
    fn index_mut(&mut self, index: JudgementLocation) -> &mut Self::Output {
        todo!("Mutable Index for The Domain.")
    }
}

impl TheDomain {
    pub(super) fn new() -> Self {
        Self {
            context_tree: ContextTree::root(),
            term_data: TermArena::new(),
        }
    }

    pub(super) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        todo!()
    }

    pub(super) fn push_variable(&mut self, name: String, typ: Type) -> FreeVariable {
        todo!()
    }

    pub(super) fn push_judgment_at(
        &mut self,
        judgement_type: JudgementType,
        judgement_location: &JudgementLocation,
    ) {
        todo!()
    }
}
