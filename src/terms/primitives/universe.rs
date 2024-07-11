use std::fmt::Display;

use crate::{deduction::judgement::Judgement, terms::Type};

pub(crate) type UniverseLevel = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Universe {
    level: UniverseLevel,
}

impl Into<Type> for Universe {
    fn into(self) -> Type {
        Type::Universe(self)
    }
}

impl Into<Judgement> for Universe {
    fn into(self) -> Judgement {
        Judgement::Type(Type::Universe(self))
    }
}

impl Universe {
    pub(crate) fn new(level: UniverseLevel) -> Self {
        Self { level }
    }

    pub(crate) fn typ(&self) -> Self {
        Self {
            level: self.level + 1,
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "𝒰_{}", self.level)
    }
}