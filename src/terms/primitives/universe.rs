use std::fmt::Display;

use super::super::types::Type;
use crate::deduction::judgement::JudgementType;

pub(crate) type UniverseLevel = usize;

pub const UNIVERSE_DISPLAY: &'static str = "𝒰";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Universe {
    level: UniverseLevel,
}

impl Into<Type> for Universe {
    fn into(self) -> Type {
        //Type::Universe(self)
        todo!()
    }
}

impl Into<JudgementType> for Universe {
    fn into(self) -> JudgementType {
        //JudgementType::Type(Type::Universe(self))
        todo!()
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
