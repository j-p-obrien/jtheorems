use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    context::Context,
    terms::{primitives::NaturalType, Term},
};

pub type Res<T> = Result<T, JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudgementType {
    WellFormed,
    Term(Term),
    Equal(Term, Term),
}

impl Display for JudgementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JudgementType::WellFormed => write!(f, "ctx"),
            JudgementType::Term(term) => todo!(),
            JudgementType::Equal(_, _) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgement {
    context: Context,
    judgement: JudgementType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JError {
    Illegal,
    NameTaken(String),
}

impl Display for Judgement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", &self.context, &self.judgement)
    }
}

impl Judgement {
    /// Creates the Empty Context with a WellFormed JudgementType.
    ///
    /// This is the starting point for all proofs.
    pub fn empty() -> Self {
        let context = Context::new();
        Self {
            context,
            judgement: JudgementType::WellFormed,
        }
    }

    pub fn extend_context(&self, name: String) -> Res<Self> {
        todo!()
    }

    pub fn natural_formation(&self) -> Res<Self> {
        todo!()
    }
}
