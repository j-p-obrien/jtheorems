use super::{
    defined::Defined,
    lambda::PiType,
    primitives::{
        coproduct::CoproductType, empty::EmptyType, identity::IdentityType, naturals::NaturalType,
        pair::SigmaType, unit::UnitType, universe::Universe,
    },
    variable::{BoundVariable, FreeVariable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    FreeVariable(FreeVariable),
    BoundVariable(BoundVariable),
    Defined(Defined),
    Universe(Universe),
    PiType(PiType),
    SigmaType(SigmaType),
    CoproductType(CoproductType),
    EmptyType(EmptyType),
    UnitType(UnitType),
    NaturalType(NaturalType),
    IdentityType(IdentityType),
}

impl Type {
    pub(super) fn is_universe(&self) -> bool {
        match self {
            Type::Universe(_) => true,
            _ => false,
        }
    }
}
