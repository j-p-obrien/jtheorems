use super::{
    coproduct::CoproductType,
    empty::EmptyType,
    identity::IdentityType,
    lambda::{FunctionType, PiType},
    naturals::NaturalType,
    pair::{ProductType, SigmaType},
    terms::*,
    unit::UnitType,
    universe::Universe,
    variable::{BoundVariable, FreeVariable},
};

#[derive(Debug)]
pub(super) struct TypePtr {
    ptr: Ptr,
}

#[derive(Debug)]
pub(super) enum Type {
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
