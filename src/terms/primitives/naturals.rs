use std::fmt::Display;

use crate::{deduction::judgement::Judgement, terms::Type};

use super::universe::Universe;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalType;

impl Into<Type> for NaturalType {
    fn into(self) -> Type {
        Type::NaturalType(self)
    }
}

impl Into<Judgement> for NaturalType {
    fn into(self) -> Judgement {
        Judgement::Type(Type::NaturalType(self))
    }
}

impl Display for NaturalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ℕ")
    }
}

impl NaturalType {
    pub(crate) fn typ(&self) -> Universe {
        Universe::new(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zero();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Succ();