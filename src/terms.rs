use std::{
    cell::RefCell,
    fmt::{write, Display},
    rc::Rc,
};

use crate::context::GlobalContext;

use self::{
    application::{Application, ApplicationData},
    lambda::{Lambda, LambdaData},
    primitives::{
        CoproductType, CoproductTypeData, EmptyType, IdentityType, IdentityTypeData, Left,
        LeftData, NaturalType, Pair, PairData, PiType, PiTypeData, Refl, ReflData, Right,
        RightData, SigmaType, SigmaTypeData, Succ, SuccData, Unit, UnitType, Universe, Zero,
    },
    variable::{Variable, VariableData},
};

pub mod application;
pub mod defined;
pub mod lambda;
pub mod primitives;
pub mod variable;

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
    Succ(SuccData),
    IdentityType(IdentityTypeData),
    Refl(ReflData),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Variable(Variable),
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

impl Display for TermData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Term {
    pub(crate) fn is_type(&self) -> bool {
        todo!()
    }

    pub(crate) fn is_universe(&self) -> bool {
        todo!()
    }

    pub(crate) fn typ(&self) -> Term {
        todo!()
    }

    pub(crate) fn substitute(&self, term: &Term) -> Term {
        todo!()
    }

    pub(crate) fn data(&self, global_context: Rc<RefCell<GlobalContext>>) -> TermData {
        todo!()
    }
}
