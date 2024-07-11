use std::fmt::Display;

const UNIT: &str = "⋆";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit();

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "⋆")
    }
}
