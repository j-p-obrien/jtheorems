use super::types::TypePtr;

type Index = u32;

#[derive(Debug)]
pub(crate) struct BoundVariable {
    index: Index,
    typ: TypePtr,
}

#[derive(Debug)]
pub(crate) struct FreeVariable {
    index: Index,
    typ: TypePtr,
}
