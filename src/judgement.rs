use std::{fmt::Display, hint::unreachable_unchecked, time::Duration};

use crate::{
    context::ContextTree,
    terms::{
        primitives::{
            naturals::NaturalType,
            universe::{Universe, UniverseLevel},
        },
        variable::VariableData,
        Term, Type,
    },
};

pub type JResult = Result<(), JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Judgement {
    WellFormed,
    Term(Term),
    Type(Type),
    EqualTerms(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContextPtr(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Judgement is a JudgementType along with its associated Context. ContextIdx points to the
/// index of the ContextTree that we are currently focusing on i.e. the rightmost variable in a Context.
pub struct Deduction {
    context_tree: ContextTree,
    context_ptr: ContextPtr,
    judgement: Judgement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JError {
    Illegal(&'static str),
    NameTaken(String),
}

impl ContextPtr {
    fn new() -> Self {
        Self(0)
    }
}

impl Judgement {
    #[inline]
    fn replace_with_wellformed(&mut self) -> Self {
        self.replace(Self::WellFormed)
    }

    #[inline]
    fn replace(&mut self, judgement: Self) -> Self {
        std::mem::replace(self, judgement)
    }
}

impl Display for Deduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Deduction {
    /// Creates the Empty Context with a WellFormed Judgement.
    ///
    /// This is the starting point for all proofs.
    pub fn new() -> Self {
        Self {
            context_tree: ContextTree::new(),
            context_ptr: ContextPtr::new(),
            judgement: Judgement::WellFormed,
        }
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant, or the name is taken we return an error.
    pub fn variable_introduction(&mut self, name: String) -> JResult {
        // This looks weird because we do not have ownership over the Judgement.
        match &self.judgement {
            Judgement::Type(_) if !self.context_tree.contains_name_at(&name, self.context_ptr) => {
                let Judgement::Type(typ) = self.judgement.replace_with_wellformed() else {
                    // SAFETY: We just checked that the variant was Type above.
                    unsafe { unreachable_unchecked() }
                };
                let variable = VariableData::new(name, typ);
                todo!()
            }
            _ => Err(JError::Illegal(
                "Judgement must be a Type to introduce a Variable.",
            )),
        }
    }

    /// Forms the Natural Type.
    ///
    /// This can be done in any WellFormed context.
    pub fn natural_formation(&mut self) -> JResult {
        match self.judgement {
            Judgement::WellFormed => {
                self.judgement = NaturalType.into();
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement must be Well Formed to introduce a Type.",
            )),
        }
    }

    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult {
        if let Judgement::WellFormed = self.judgement {
            self.judgement = Universe::new(level).into();
            Ok(())
        } else {
            Err(JError::Illegal(
                "Judgement must be Well Formed to introduce a Universe.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_introduction() {
        let mut deduction = Deduction::new();
        let _ = deduction.universe_formation(0);
        assert_eq!(deduction.variable_introduction("x".to_string()), Ok(()));
        assert_eq!(
            deduction.variable_introduction("x".to_string()),
            Err(JError::NameTaken("x".to_string()))
        );
    }

    #[test]
    fn test_natural_formation() {
        let mut deduction = Deduction::new();
        assert_eq!(deduction.natural_formation(), Ok(()));
        assert_eq!(
            deduction.judgement,
            Judgement::Type(Type::NaturalType(NaturalType))
        );
    }

    #[test]
    fn test_universe_formation() {
        let mut deduction = Deduction::new();
        assert_eq!(deduction.universe_formation(0), Ok(()));
        assert_eq!(
            deduction.judgement,
            Judgement::Type(Type::Universe(Universe::new(0)))
        );
    }
}
