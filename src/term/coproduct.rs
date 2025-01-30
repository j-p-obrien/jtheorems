use super::{
    terms::TermPtr,
    types::{Type, TypePtr},
};

pub(super) struct InL {
    data: TermPtr,
    typ: TypePtr,
}

pub(super) struct InR {
    data: TermPtr,
    typ: TypePtr,
}

pub(super) struct CoproductType {
    left: TypePtr,
    right: TypePtr,
}
