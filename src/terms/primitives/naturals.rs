use std::fmt::Display;

use crate::{deduction::judgement::JudgementType, terms::Term};

use super::{super::types::Type, universe::Universe};

const NATURAL_DISPLAY: &str = "ℕ";
const ZERO_DISPLAY: &str = "0";
const SUCC_DISPLAY: &str = "succ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zero();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Succ();

impl Display for NaturalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{NATURAL_DISPLAY}")
    }
}

impl From<NaturalType> for Type {
    fn from(value: NaturalType) -> Self {
        Type::NaturalType(value)
    }
}

impl From<NaturalType> for JudgementType {
    fn from(value: NaturalType) -> Self {
        JudgementType::Type(Type::NaturalType(value))
    }
}

impl From<Zero> for Term {
    fn from(value: Zero) -> Self {
        Term::Zero(value)
    }
}

impl From<Zero> for JudgementType {
    fn from(value: Zero) -> Self {
        JudgementType::Term(Term::Zero(value))
    }
}

impl From<Succ> for Term {
    fn from(value: Succ) -> Self {
        Term::Succ(value)
    }
}

impl From<Succ> for JudgementType {
    fn from(value: Succ) -> Self {
        JudgementType::Term(Term::Succ(value))
    }
}

impl NaturalType {
    pub(crate) fn typ(&self) -> Universe {
        Universe::new(0)
    }
}
