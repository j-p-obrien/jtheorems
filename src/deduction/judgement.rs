use std::{fmt::Display, hint::unreachable_unchecked};

use crate::{
    deduction::the_domain::TheDomain,
    terms::{
        primitives::{
            naturals::NaturalType,
            universe::{Universe, UniverseLevel},
        },
        variable::VariableData,
        Term, Type,
    },
};

type JudgementPtrSize = usize;

pub type JResult = Result<(), JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgementKind {
    WellFormed,
    Term(Term),
    Type(Type),
    EqualTerms(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgementPtr {
    WellFormed,
    Term(JudgementPtrSize),
    Type(JudgementPtrSize),
    EqualTerms(JudgementPtrSize),
    EqualTypes(JudgementPtrSize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContextPtr(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Judgement {
    context_ptr: ContextPtr,
    judgement_kind: JudgementKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Judgement is a JudgementType along with its associated Context. ContextIdx points to the
/// index of the ContextTree that we are currently focusing on i.e. the rightmost variable in a Context.
pub struct Deduction {
    domain: TheDomain,
    judgement: Judgement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JError {
    Illegal(&'static str),
    NameTaken(String),
}

impl ContextPtr {
    /// Creates a ContextPtr that points to the root of the ContextTree i.e. at index 0.
    fn empty_context() -> Self {
        Self(0)
    }
}

impl JudgementPtr {
    fn well_formed() -> Self {
        Self::WellFormed
    }
}

impl JudgementKind {
    fn well_formed() -> Self {
        Self::WellFormed
    }

    #[inline]
    fn replace_with_wellformed(&mut self) -> Self {
        self.replace(Self::WellFormed)
    }

    #[inline]
    fn replace(&mut self, judgement: Self) -> Self {
        std::mem::replace(self, judgement)
    }
}

impl Judgement {
    pub(crate) fn new() -> Self {
        Self {
            judgement_kind: JudgementKind::well_formed(),
            context_ptr: ContextPtr::empty_context(),
        }
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
            domain: TheDomain::new(),
            judgement: Judgement::new(),
        }
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant, or the name is taken we return an error.
    pub fn variable_introduction(&mut self, name: String) -> JResult {
        todo!("Variable Introduction")
    }

    /// Forms the Natural Type.
    ///
    /// This can be done in any WellFormed context.
    pub fn natural_formation(&mut self) -> JResult {
        todo!("Natural Formation")
    }

    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult {
        todo!("Universe Formation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
