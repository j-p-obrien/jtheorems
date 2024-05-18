use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    context::{Context, GlobalContext, GLOBAL_CONTEXT_CAPACITY},
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
    pub fn empty() -> Self {
        let global_context = GlobalContext::new(GLOBAL_CONTEXT_CAPACITY);
        let context = Context::new(Rc::new(RefCell::new(global_context)));
        Self {
            context,
            judgement: JudgementType::WellFormed,
        }
    }

    pub fn extend_context(&self, name: String) -> Res<Self> {
        todo!()
    }

    pub fn natural_formation(&self) -> Res<Self> {
        match self.judgement {
            JudgementType::WellFormed => {
                let new_context = self.context.clone();
                Ok(Self {
                    context: new_context,
                    judgement: JudgementType::Term(Term::NaturalType(NaturalType)),
                })
            }
            _ => Err(JError::Illegal),
        }
    }
}
