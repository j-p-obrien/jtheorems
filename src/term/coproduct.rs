use super::{
    terms::TermPtr,
    types::{Type, TypePtr},
};

#[derive(Debug)]
pub(super) struct InL {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug)]
pub(super) struct InR {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug)]
pub(super) struct CoproductType {
    left: TypePtr,
    right: TypePtr,
}
