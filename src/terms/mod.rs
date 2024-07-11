use lambda::{PiType, PiTypeData};
use primitives::{
    coproduct::{CoproductType, Left, Right, RightData},
    identity::{IdentityType, Refl, ReflData},
    naturals::{Succ, Zero},
    pair::{Pair, SigmaType, SigmaTypeData},
    unit::{Unit, UnitType},
    universe::Universe,
};

use self::{
    application::{Application, ApplicationData},
    lambda::{Lambda, LambdaData},
    primitives::coproduct::CoproductTypeData,
    primitives::coproduct::LeftData,
    primitives::empty::EmptyType,
    primitives::identity::IdentityTypeData,
    primitives::naturals::NaturalType,
    primitives::pair::PairData,
    variable::{FreeVariable, VariableData},
};

pub mod application;
pub mod defined;
pub mod lambda;
pub mod primitives;
pub mod variable;

pub(crate) type TermIdx = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermData2 {
    Variable(VariableData),
    Lambda(LambdaData),
    Application(ApplicationData),
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
    //NaturalType(NaturalType),
    //Zero(Zero),
    //Succ(Succ),
    IdentityType(IdentityTypeData),
    Refl(ReflData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermData {
    Variable(VariableData),
    Lambda(LambdaData),
    Application(ApplicationData),
    Universe(Universe),
    PiType(PiTypeData),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term2 {
    Variable(FreeVariable),
    Lambda(Lambda),
    Application(Application),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    data: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Universe(Universe),
    Variable(FreeVariable),
    PiType(PiType),
    SigmaType(SigmaType),
    CoproductType(CoproductType),
    EmptyType(EmptyType),
    UnitType(UnitType),
    NaturalType(NaturalType),
    IdentityType(IdentityType),
}

impl Term {
    pub(crate) fn new() -> Self {
        todo!()
    }

    pub(crate) fn is_type(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_universe(&self) -> bool {
        todo!()
    }

    pub(crate) fn typ(&self) -> &Term {
        todo!()
    }

    pub(crate) fn substitute(&self, term: &Term) -> Term {
        todo!()
    }

    pub(crate) fn data(&self) -> TermData {
        todo!()
    }

    pub(crate) fn id(&self) -> usize {
        todo!()
    }
}
