use std::ops::{Index, IndexMut};

use crate::terms::{primitives::naturals::NaturalType, types::Type, variable::FreeVariable};

use super::{
    context_tree::{ContextPtr, ContextTree},
    judgement::{Judgement, JudgementType},
    term_arena::TermArena,
};

/// The Domain is the central repository of all Terms and their associated Context. This data
/// structure is the heart of the proof system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TheDomain {
    context_tree: ContextTree,
    term_data: TermArena,
}

impl TheDomain {
    pub(super) fn new() -> Self {
        Self {
            context_tree: ContextTree::root(),
            term_data: TermArena::new(),
        }
    }

    pub(super) fn contains_name_at(&self, name: &str, location: &ContextPtr) -> bool {
        todo!()
    }

    pub(super) fn push_variable(&mut self, name: String, typ: Type) -> FreeVariable {
        todo!()
    }

    pub(super) fn push_natural_type_at(&mut self, context: &ContextPtr) -> Judgement {
        let naturals: JudgementType = NaturalType.into();
        // TODO: Decide whether or not to actually push this into the Context Tree. Naturals are
        // size zero and can be formed in any Context anyways.
        self.context_tree[*context].push(naturals.clone());
        Judgement::new(*context, naturals)
    }
}
