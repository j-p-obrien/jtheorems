pub mod application;
pub mod defined;
pub mod lambda;
pub mod primitives;
pub mod variable;

use application::{Application, ApplicationData};
use defined::{Defined, DefinedData};
use lambda::{Lambda, LambdaData, PiType, PiTypeData};
use primitives::{
    coproduct::{CoproductType, CoproductTypeData, Left, LeftData, Right, RightData},
    empty::EmptyType,
    identity::{IdentityType, IdentityTypeData, Refl, ReflData},
    naturals::{NaturalType, Succ, Zero},
    pair::{Pair, PairData, SigmaType, SigmaTypeData},
    unit::{Unit, UnitType},
    universe::Universe,
};
use variable::{FreeVariable, VariableData};

pub(crate) type TermPtr = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermData2 {
    Variable(VariableData),
    Lambda(LambdaData),
    Application(ApplicationData),
    Defined(DefinedData),
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
    Defined(DefinedData),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    data: TermPtr,
}

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
