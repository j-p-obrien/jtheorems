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
    variable::{FreeVariable, VariableData},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Variable(FreeVariable),
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
