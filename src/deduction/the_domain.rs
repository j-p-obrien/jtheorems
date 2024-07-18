use crate::terms::{primitives::naturals::NaturalType, types::Type};

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
            context_tree: ContextTree::new(),
            term_data: TermArena::new(),
        }
    }

    pub(super) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        self.context_tree
            .contains_name_at(name, location, &self.term_data)
    }

    pub(super) fn variable_intro_at(
        &mut self,
        name: String,
        typ: Type,
        location: ContextPtr,
    ) -> Judgement {
        let variable = self.term_data.push_variable(name, typ);
        todo!()
    }

    pub(super) fn form_natural_type_at(&mut self, context: ContextPtr) -> Judgement {
        let naturals: JudgementType = NaturalType.into();
        // TODO: Decide whether or not to actually push this into the Context Tree. Naturals are
        // size zero and can be formed in any Context anyways.
        self.context_tree.push_at(context, naturals.clone());
        Judgement::new(context, naturals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_formation() {
        let mut domain = TheDomain::new();
        let judgement = domain.form_natural_type_at(ContextPtr::empty_context());
        assert_eq!(judgement.judgement_type(), &NaturalType.into());
    }
}
