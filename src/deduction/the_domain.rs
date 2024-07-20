use crate::term::{
    primitives::{
        naturals::{NaturalType, Zero},
        universe::{Universe, UniverseLevel},
    },
    types::Type,
};

use super::{
    context_tree::{ContextPtr, ContextTree},
    error::JError,
    judgement::{Judgement, JudgementType},
    term_arena::TermArena,
    terminal::JResult,
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

    pub(super) fn variable_introduction_at(
        &mut self,
        name: String,
        typ: Type,
        context: ContextPtr,
    ) -> Judgement {
        let variable = self.term_data.add_variable(name, typ);
        let new_context = self
            .context_tree
            .variable_introduction_at(variable, context);
        Judgement::well_formed_at(new_context)
    }

    pub(super) fn try_variable_introduction_at(
        &mut self,
        name: String,
        typ: Type,
        context: ContextPtr,
    ) -> JResult<Judgement> {
        if self
            .context_tree
            .contains_name_at(&name, context, &self.term_data)
        {
            return Err(JError::NameTaken(name));
        }
        Ok(self.variable_introduction_at(name, typ, context))
    }

    pub(super) fn try_variable_rule_at(
        &mut self,
        name: &str,
        context: ContextPtr,
    ) -> JResult<Judgement> {
        let variable = self
            .context_tree
            .try_variable_rule_at(name, context, &self.term_data)?;
        Ok(Judgement::new(context, variable.into()))
    }

    pub(super) fn natural_formation_at(&mut self, context: ContextPtr) -> Judgement {
        let naturals: JudgementType = NaturalType.into();
        // TODO: Decide whether or not to actually push this into the Context Tree. Naturals are
        // size zero and can be formed in any Context anyways.
        self.context_tree
            .add_judgement_at(naturals.clone(), context);
        Judgement::new(context, naturals)
    }

    pub(super) fn zero_formation_at(&mut self, context: ContextPtr) -> Judgement {
        let zero: JudgementType = Zero.into();
        // TODO: Decide whether or not to actually push this into the Context Tree. Zero is
        // size zero and can be formed in any Context anyways.
        self.context_tree.add_judgement_at(zero.clone(), context);
        Judgement::new(context, zero)
    }

    pub(super) fn universe_formation_at(
        &mut self,
        level: UniverseLevel,
        context: ContextPtr,
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
        let judgement = domain.natural_formation_at(ContextPtr::empty_context());
        assert_eq!(judgement.judgement_type(), &NaturalType.into());
    }
}
