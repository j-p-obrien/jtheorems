use crate::{Variable, VariableData, R};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitType();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiType {
    input: Variable,
    output_typ: Type,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Universe {
    level: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypeData {
    Variable(VariableData),
    Empty(EmptyType),
    Unit(UnitType),
    Pi(PiType),
    Universe(Universe),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    data: R<TypeData>,
}

impl From<EmptyType> for TypeData {
    fn from(value: EmptyType) -> Self {
        value.into()
    }
}

impl From<PiType> for TypeData {
    fn from(value: PiType) -> Self {
        Self::Pi(value)
    }
}

impl From<Universe> for TypeData {
    fn from(value: Universe) -> Self {
        Self::Universe(value)
    }
}

impl<T> From<T> for Type
where
    T: Into<TypeData>,
{
    fn from(value: T) -> Self {
        value.into().into()
    }
}

impl PiType {
    fn formation(variable: Variable, output_typ: Type) -> Self {
        todo!()
    }
}

impl Type {
    pub fn is_small(&self) -> bool {
        match &*(*self.data).borrow() {
            TypeData::Universe(_) => false,
            _ => true,
        }
    }

    pub fn is_universe(&self) -> bool {
        match &*(*self.data).borrow() {
            TypeData::Universe(_) => true,
            _ => false,
        }
    }

    fn pi_formation(&self, variable: Variable, typ: Type) -> Self {
        let pitypedata = PiType::formation(variable, typ);
        pitypedata.into()
    }
}
