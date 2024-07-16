use super::context_tree::ContextPtr;
use crate::terms::{types::Type, Term};

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
    context: ContextPtr,
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
            context,
            judgement_type,
        }
    }

    pub(super) fn well_formed_empty_context() -> Self {
        Self {
            context: ContextPtr::empty_context(),
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn judgement_type(&self) -> &JudgementType {
        &self.judgement_type
    }

    pub(super) fn context(&self) -> &ContextPtr {
        &self.context
    }
}
