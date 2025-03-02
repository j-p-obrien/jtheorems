use super::{
    app::Application,
    constant::Constant,
    coproduct::{CoproductType, InL, InR},
    empty::EmptyType,
    identity::{IdentityType, Refl},
    lambda::{FunctionType, Lambda, PiType},
    naturals::{NaturalType, Succ, Zero},
    pair::{Pair, ProductType, SigmaType},
    unit::{Unit, UnitType},
    universe::Universe,
    variable::*,
};

pub(super) type Ptr = u32;

#[derive(Debug)]
pub(super) struct TermPtr {
    ptr: Ptr,
}

#[derive(Debug)]
pub(crate) enum Term {
    Application(Application),
    Constant(Constant),
    InL(InL),
    InR(InR),
    CoproductType(CoproductType),
    EmptyType(EmptyType),
    Refl(Refl),
    IdentityType(IdentityType),
    Lambda(Lambda),
    FunctionType(FunctionType),
    PiType(PiType),
    Zero(Zero),
    Succ(Succ),
    NaturalType(NaturalType),
    Pair(Pair),
    ProductType(ProductType),
    SigmaType(SigmaType),
    Unit(Unit),
    UnitType(UnitType),
    Universe(Universe),
    BoundVariable(BoundVariable),
    FreeVariable(FreeVariable),
}
