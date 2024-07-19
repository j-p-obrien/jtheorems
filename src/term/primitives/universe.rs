use std::fmt::Display;

use super::super::types::Type;
use crate::deduction::judgement::JudgementType;

pub(crate) type UniverseLevel = usize;

pub const UNIVERSE_DISPLAY: &'static str = "𝒰";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Universe {
    level: UniverseLevel,
}

impl From<Universe> for Type {
    fn from(universe: Universe) -> Self {
        Type::Universe(universe)
    }
}

impl From<Universe> for JudgementType {
    fn from(universe: Universe) -> Self {
        JudgementType::Type(Type::Universe(universe))
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
        write!(f, "{}_{}", UNIVERSE_DISPLAY, self.level)
    }
}
