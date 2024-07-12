use crate::terms::{types::Type, Term};

pub type ContextPtrSize = usize;
pub type JudgementPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContextPtr(ContextPtrSize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JudgementPtr(JudgementPtrSize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct JudgementLocation {
    context_ptr: ContextPtr,
    judgement_ptr: JudgementPtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JudgementKind {
    WellFormed,
    Term(Term),
    Type(Type),
    EqualTerms(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Judgement {
    judgement_location: JudgementLocation,
    judgement_kind: JudgementKind,
}

impl ContextPtr {
    /// Creates a ContextPtr that points to the root of the ContextTree i.e. at index 0.
    pub(crate) fn empty_context() -> Self {
        Self(0)
    }
}

impl JudgementPtr {
    fn well_formed() -> Self {
        Self(0)
    }
}

impl JudgementLocation {
    pub(crate) fn empty_well_formed() -> Self {
        Self {
            context_ptr: ContextPtr::empty_context(),
            judgement_ptr: JudgementPtr::well_formed(),
        }
    }
}

impl JudgementKind {
    pub(crate) fn well_formed() -> Self {
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
            judgement_location: JudgementLocation::empty_well_formed(),
            judgement_kind: JudgementKind::well_formed(),
        }
    }
}
