use super::{
    application::{Application, ApplicationData},
    defined::{Defined, DefinedData},
    lambda::{Lambda, LambdaData, PiType, PiTypeData},
    primitives::{
        coproduct::{CoproductType, CoproductTypeData, Left, LeftData, Right, RightData},
        empty::EmptyType,
        identity::{IdentityType, IdentityTypeData, Refl, ReflData},
        naturals::{NaturalType, Succ, Zero},
        pair::{Pair, PairData, SigmaType, SigmaTypeData},
        unit::{Unit, UnitType},
        universe::Universe,
    },
    variable::{BoundVariable, FreeVariable, VariableData},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermData {
    Variable(VariableData),
    Application(ApplicationData),
    Defined(DefinedData),
    PiType(PiTypeData),
    Lambda(LambdaData),
    Universe(Universe),
    SigmaType(SigmaTypeData),
    Pair(PairData),
    CoproductType(CoproductTypeData),
    Left(LeftData),
    Right(RightData),
    EmptyType(EmptyType),
    UnitType(UnitType),
    Unit(Unit),
    NaturalType(NaturalType),
    Zero(Zero),
    Succ(Succ),
    IdentityType(IdentityTypeData),
    Refl(ReflData),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Term {
    FreeVariable(FreeVariable),
    BoundVariable(BoundVariable),
    Lambda(Lambda),
    Application(Application),
    Defined(Defined),
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
    NaturalType(NaturalType),
    Zero(Zero),
    Succ(Succ),
    IdentityType(IdentityType),
    Refl(Refl),
}
