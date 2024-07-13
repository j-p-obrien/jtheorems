use super::{context_tree::ContextPtr, judgement_tree::JudgementPtr};
use crate::terms::{types::Type, Term};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JudgementLocation {
    context_ptr: ContextPtr,
    judgement_ptr: JudgementPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum JudgementType {
    WellFormed,
    Term(Term),
    Type(Type),
    EqualTerms(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Judgement {
    judgement_location: JudgementLocation,
    judgement_type: JudgementType,
}

impl JudgementLocation {
    pub(super) fn empty_well_formed() -> Self {
        Self {
            context_ptr: ContextPtr::empty_context(),
            judgement_ptr: JudgementPtr::well_formed(),
        }
    }
}

impl JudgementType {
    pub(super) fn well_formed() -> Self {
        Self::WellFormed
    }

    #[inline]
    fn replace_with_wellformed(&mut self) -> Self {
        std::mem::replace(self, Self::WellFormed)
    }

    #[inline]
    fn replace(&mut self, judgement: Self) -> Self {
        std::mem::replace(self, judgement)
    }
}

impl Judgement {
    pub(super) fn new() -> Self {
        Self {
            judgement_location: JudgementLocation::empty_well_formed(),
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn judgement_kind(&self) -> &JudgementType {
        &self.judgement_type
    }

    pub(super) fn judgement_location(&self) -> &JudgementLocation {
        &self.judgement_location
    }
}
