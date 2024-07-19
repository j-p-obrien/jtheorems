use super::context_tree::Context;
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
    context_ptr: Context,
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
    pub(super) fn new(context: Context, judgement_type: JudgementType) -> Self {
        Self {
            context_ptr: context,
            judgement_type,
        }
    }

    pub(super) fn well_formed_empty_context() -> Self {
        Self {
            context_ptr: Context::empty_context(),
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn well_formed_at(context: Context) -> Self {
        Self {
            context_ptr: context,
            judgement_type: JudgementType::well_formed(),
        }
    }

    pub(super) fn judgement_type(&self) -> &JudgementType {
        &self.judgement_type
    }

    pub(super) fn context(&self) -> Context {
        self.context_ptr
    }
}
