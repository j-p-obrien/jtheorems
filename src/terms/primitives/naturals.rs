use std::fmt::Display;

use crate::{deduction::judgement::JudgementType, terms::Term};

use super::{super::types::Type, universe::Universe};

const NATURAL_DISPLAY: &str = "ℕ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zero();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Succ();

impl Into<Type> for NaturalType {
    fn into(self) -> Type {
        Type::NaturalType(self)
    }
}

impl Into<JudgementType> for NaturalType {
    fn into(self) -> JudgementType {
        JudgementType::Type(Type::NaturalType(self))
    }
}

impl Display for NaturalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NATURAL_DISPLAY}")
    }
}

impl NaturalType {
    pub(crate) fn typ(&self) -> Universe {
        Universe::new(0)
    }
}

impl Into<Term> for Zero {
    fn into(self) -> Term {
        Term::Zero(self)
    }
}

impl Into<JudgementType> for Zero {
    fn into(self) -> JudgementType {
        JudgementType::Term(Term::Zero(self))
    }
}
