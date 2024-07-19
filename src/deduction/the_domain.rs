use crate::terms::{
    primitives::{
        naturals::NaturalType,
        universe::{Universe, UniverseLevel},
    },
    types::Type,
};

use super::{
    context_tree::{Context, ContextTree},
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

    pub(super) fn contains_name_at(&self, name: &str, context: Context) -> bool {
        self.context_tree
            .contains_name_at(name, &self.term_data, context)
    }

    pub(super) fn variable_introduction_at(
        &mut self,
        name: String,
        typ: Type,
        context: Context,
    ) -> Judgement {
        let variable = self.term_data.add_variable(name, typ);
        let new_context = self
            .context_tree
            .variable_introduction_at(variable, context);
        Judgement::well_formed_at(new_context)
    }

    pub(super) fn natural_formation_at(&mut self, context: Context) -> Judgement {
        let naturals: JudgementType = NaturalType.into();
        // TODO: Decide whether or not to actually push this into the Context Tree. Naturals are
        // size zero and can be formed in any Context anyways.
        self.context_tree
            .add_judgement_at(naturals.clone(), context);
        Judgement::new(context, naturals)
    }

    pub(super) fn universe_formation_at(
        &mut self,
        level: UniverseLevel,
        context: Context,
    ) -> Judgement {
        let universe: JudgementType = Universe::new(level).into();
        self.context_tree
            .add_judgement_at(universe.clone(), context);
        Judgement::new(context, universe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_formation() {
        let mut domain = TheDomain::new();
        let judgement = domain.natural_formation_at(Context::empty_context());
        assert_eq!(judgement.judgement_type(), &NaturalType.into());
    }
}
