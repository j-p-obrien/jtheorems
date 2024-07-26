use crate::term::{
    primitives::{
        naturals::{NaturalType, Zero},
        universe::{Universe, UniverseLevel},
    },
    types::Type,
    variable::{FreeVariable, VariableData},
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

    fn contains_name_at(&self, name: &str, context: ContextPtr) -> bool {
        let mut current = context;
        while !current.is_empty_context() {
            // SAFETY: We just checked that the context is non-empty.
            let nonempty_context = unsafe { self.context_tree.get_nonempty_unchecked(current) };
            if nonempty_context.variable().has_name(name, &self.term_data) {
                return true;
            }
            current = nonempty_context.parent();
        }
        false
    }

    fn context_extension_at(&mut self, name: String, typ: Type, context: ContextPtr) -> Judgement {
        let data = VariableData::new(name, typ).into();
        let variable: FreeVariable = self.term_data.add_term(data);
        let new_context = self.context_tree.context_extension_at(variable, context);
        Judgement::well_formed_at(new_context)
    }

    pub(super) fn try_context_extension_at(
        &mut self,
        name: String,
        typ: Type,
        context: ContextPtr,
    ) -> JResult<Judgement> {
        if self.contains_name_at(&name, context) {
            return Err(JError::NameTaken(name));
        }
        Ok(self.context_extension_at(name, typ, context))
    }

    pub(super) fn try_variable_introduction_at(
        &mut self,
        name: &str,
        context: ContextPtr,
    ) -> JResult<Judgement> {
        let variable =
            self.context_tree
                .try_variable_introduction_at(name, context, &self.term_data)?;
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

    pub(super) fn zero_introduction_at(&mut self, context: ContextPtr) -> Judgement {
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

    pub(super) fn try_pi_formation_at(
        &mut self,
        typ: Type,
        context: ContextPtr,
    ) -> JResult<Judgement> {
        if context.is_empty_context() {
            return Err(JError::Illegal(
                "Context must not be empty to form a Product Type.",
            ));
        }
        let pi_type = self
            .context_tree
            .pi_formation_at(typ, context, &mut self.term_data);
        todo!("Implement try_pi_formation_at.");
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
