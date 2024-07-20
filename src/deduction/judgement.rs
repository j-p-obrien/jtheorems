use super::context_tree::ContextPtr;
use crate::term::{types::Type, Term};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgementType {
    WellFormed,
    Term(Term),
    Type(Type),
    EqualTerms(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgement {
    context_ptr: ContextPtr,
    judgement_type: JudgementType,
}

impl JudgementType {
    pub(super) fn well_formed() -> Self {
        Self::WellFormed
    }

    fn replace_with_wellformed(&mut self) -> Self {
        std::mem::replace(self, Self::WellFormed)
    }

    fn replace(&mut self, judgement: Self) -> Self {
        std::mem::replace(self, judgement)
    }
}

impl Judgement {
    pub(super) fn new(context: ContextPtr, judgement_type: JudgementType) -> Self {
        Self {
            context_ptr: context,
            judgement_type,
        }
    }

    pub(super) fn well_formed_empty_context() -> Self {
        Self {
            context_ptr: ContextPtr::empty_context(),
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn well_formed_at(context: ContextPtr) -> Self {
        Self {
            context_ptr: context,
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn judgement_type(&self) -> &JudgementType {
        &self.judgement_type
    }

    pub(super) fn context(&self) -> ContextPtr {
        self.context_ptr
    }

    /// Changes the JudgementType to WellFormed without changing the Context.
    pub(super) fn well_formed_in_context(&mut self) {
        self.judgement_type = JudgementType::WellFormed;
    }
}
