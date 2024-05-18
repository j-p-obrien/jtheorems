use std::fmt::Display;

use crate::terms::{Term, TermData};

use super::variable::Variable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Universe {
    level: usize,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "𝒰_{}", self.level)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiTypeData {
    variable: Variable,
    output_typ: Term,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiType {
    /// Points to a PiTypeData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaTypeData {
    variable: Variable,
    output_typ: Term,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaType {
    /// Points to a SigmaTypeData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairData {
    left: Term,
    right: Term,
    typ: SigmaType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    /// Points to a PairData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoproductTypeData {
    left: Term,
    right: Term,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoproductType {
    /// Points to a CoproductTypeData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Left {
    /// Points to a LeftData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Right {
    /// Points to a RightData
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalType;

impl Display for NaturalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ℕ")
    }
}

impl NaturalType {
    pub(crate) fn typ(&self) -> Universe {
        Universe { level: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zero();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccData {
    input: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succ {
    /// Points to a SuccData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityTypeData {
    left: Term,
    right: Term,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityType {
    /// Points to an IdentityTypeData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflData {
    term: Term,
    typ: IdentityType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refl {
    /// Points to a ReflData
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveData {
    //Universe(Universe),
    PiType(PiTypeData),
    SigmaType(SigmaTypeData),
    Pair(PairData),
    CoproductType(CoproductTypeData),
    Left(LeftData),
    Right(RightData),
    //EmptyType(EmptyType),
    //UnitType(UnitType),
    //Unit(Unit),
    //NaturalNumbers(NaturalType),
    //Zero(Zero),
    Succ(SuccData),
    IdentityType(IdentityTypeData),
    Refl(ReflData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Universe(Universe),
    PiType(PiType),
    SigmaType(SigmaType),
    Pair(Pair),
    CoproductType(CoproductType),
    Left(Left),
    Right(Right),
    EmptyType(EmptyType),
    UnitType(UnitType),
    Unit(Unit),
    NaturalNumbers(NaturalType),
    Zero(Zero),
    Succ(Succ),
    IdentityType(IdentityType),
    Refl(Refl),
}

impl Primitive {
    fn is_type(&self) -> bool {
        match self {
            Primitive::Universe(_) => true,
            Primitive::PiType(_) => true,
            Primitive::SigmaType(_) => true,
            Primitive::CoproductType(_) => true,
            Primitive::UnitType(_) => true,
            Primitive::EmptyType(_) => true,
            Primitive::NaturalNumbers(_) => true,
            Primitive::IdentityType(_) => true,
            _ => false,
        }
    }
}
