use crate::deduction::term_arena::TermArena;

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
    pub(super) fn name_is_taken(&self, name: &str, term_data: &TermArena) -> bool {
        match self {
            Type::FreeVariable(free_variable) => free_variable.name_is_taken(name, term_data),
            Type::BoundVariable(bound_variable) => bound_variable.name_is_taken(name, term_data),
            Type::Defined(_) => todo!("Determine how to deal with names for Defined types."),
            _ => false,
        }
    }
}
