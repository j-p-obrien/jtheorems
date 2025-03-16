use super::{terms::Term, types::Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO: Add support for polymorphic levels
enum UniverseLevel {
    Definite(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Universe {
    level: UniverseLevel,
}

impl From<Universe> for Type {
    fn from(value: Universe) -> Self {
        Type::Universe(value)
    }
}

impl From<Universe> for Term {
    fn from(value: Universe) -> Self {
        Term::Universe(value)
    }
}

impl Universe {
    pub(crate) fn new(level: usize) -> Self {
        Self {
            level: UniverseLevel::Definite(level),
        }
    }
}
