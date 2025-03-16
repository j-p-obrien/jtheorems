use crate::terminal::context::ContextError;

use super::{
    coproduct::CoproductType,
    empty::EmptyType,
    identity::IdentityType,
    lambda::{FunctionType, PiType},
    naturals::NaturalType,
    pair::{ProductType, SigmaType},
    terms::Term,
    unit::UnitType,
    universe::Universe,
    variable::{BoundVariable, FreeVariable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Type {
    CoproductType(CoproductType),
    EmptyType(EmptyType),
    IdentityType(IdentityType),
    FunctionType(FunctionType),
    PiType(PiType),
    NaturalType(NaturalType),
    ProductType(ProductType),
    SigmaType(SigmaType),
    UnitType(UnitType),
    Universe(Universe),
    BoundVariable(BoundVariable),
    FreeVariable(FreeVariable),
}

impl TryFrom<Term> for Type {
    type Error = ContextError;

    fn try_from(value: Term) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Type {
    pub(crate) fn new_universe(level: usize) -> Type {
        Type::Universe(Universe::new(level))
    }
}
